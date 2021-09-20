use std::{collections::HashSet, marker::PhantomData, ops::Deref, sync::Arc};

use doxa_core::{chrono::Utc, tokio, tracing::debug};
use doxa_db::{
    diesel::PgConnection,
    model::{
        game::{Game, GameParticipant, GameParticipantUser, GameResult, InsertableGame},
        leaderboard::LeaderboardScore,
        storage::AgentUpload,
        user::User,
    },
    DieselError, PgPool,
};
use doxa_executor::{client::GameClient, event::StartEvent};
use doxa_mq::{
    model::{GameEvent, MatchRequest},
    MQPool,
};

use crate::{
    client::Competition,
    error::{AgentNotFound, ContextError, ParseSystemMessageError, StartEventNotFound},
};

// Maybe rename to BaseContext which contains stuff that can be cloned around then before actually
// passing it into things such as routes we extract the db_pool and store it in the Context, or we
// make those kinds of methods take in a DbConnection and provide another method which takes in
// DbPool and returns DbConnection
//
// Maybe make context generic over the competition to allow for automatically having the correct
// type for match request. Build one context per competiton
//
// New idea for handling DB stuff:
// allow clients to build their own db queries maybe with a replacement of action with `controller`
// apis where a struct has methods that return querys (not results). Then never give the actual db
// connection and create an exec method that takes in the query, spawns the blocking thread etc..
#[derive(Clone)]
pub struct Context<C: Competition + ?Sized> {
    mq_pool: Arc<MQPool>,
    pg_pool: Arc<PgPool>,
    competition: PhantomData<C>,
    competition_id: i32,
}

impl<C: Competition + ?Sized> Context<C> {
    pub(crate) fn new(mq_pool: Arc<MQPool>, pg_pool: Arc<PgPool>, competition_id: i32) -> Self {
        Context {
            mq_pool,
            pg_pool,
            competition: PhantomData,
            competition_id,
        }
    }
}

impl<C: Competition + ?Sized> Context<C> {
    /// This will create the game record in the database and then emit the match request event.
    ///
    /// The `GameClient` will recieve the match_request on initialization.
    pub async fn emit_match_request(
        &self,
        agents: Vec<String>,
        match_request: <C::GameClient as GameClient>::MatchRequest,
    ) -> Result<(), ContextError> {
        let db = self.db_connection().await?;
        let competition = self.competition_id;
        let game = tokio::task::spawn_blocking::<_, Result<_, DieselError>>({
            let agents = agents.clone();
            move || {
                let game = doxa_db::action::game::create_game(
                    &db,
                    &InsertableGame {
                        start_time: Utc::now(),
                        competition,
                    },
                )?;

                for agent in agents {
                    doxa_db::action::game::add_participant(
                        &db,
                        &GameParticipant {
                            agent,
                            game: game.id,
                        },
                    )?;
                }

                Ok(game)
            }
        })
        .await??;

        let connection = self.mq_pool.get().await?;

        let match_request = MatchRequest {
            agents,
            payload: match_request,
            game_id: game.id,
        };

        doxa_mq::action::emit_match_request(&connection, &match_request, C::COMPETITION_NAME)
            .await?;

        Ok(())
    }

    /// Performs `nxn` pairwise matching.
    /// This should be run whenever a new agent has been uploaded.
    /// This queue a match with all active agents uploaded after this one.
    ///
    /// If `both_directions` is set to true then for every pair of agents two matches will be
    /// created (a, b) and (b,a).
    pub async fn pair_matching<F: FnMut() -> <C::GameClient as GameClient>::MatchRequest>(
        &self,
        new_agent: String,
        both_directions: bool,
        mut match_request_generator: F,
    ) -> Result<(), ContextError> {
        let agent = self
            .get_agent(new_agent.clone())
            .await?
            .ok_or(AgentNotFound)?;

        let active_agents = self
            .run_query(move |conn| {
                doxa_db::action::storage::get_active_agents_uploaded_before(
                    conn,
                    agent.competition,
                    agent.uploaded_at,
                )
            })
            .await?;

        dbg!(&active_agents);
        for other_agent in active_agents {
            self.emit_match_request(
                vec![new_agent.clone(), other_agent.id.clone()],
                match_request_generator(),
            )
            .await?;

            if both_directions {
                self.emit_match_request(
                    vec![other_agent.id.clone(), new_agent.clone()],
                    match_request_generator(),
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn db_connection(&self) -> Result<impl Deref<Target = PgConnection>, ContextError> {
        let pool = self.pg_pool.clone();
        let connection = tokio::task::spawn_blocking(move || pool.get()).await??;

        Ok(connection)
    }

    pub(crate) async fn run_query<
        T: Send + 'static,
        F: FnOnce(&PgConnection) -> Result<T, DieselError> + Send + 'static,
    >(
        &self,
        f: F,
    ) -> Result<T, ContextError> {
        let connection = self.db_connection().await?;

        tokio::task::spawn_blocking(move || f(&connection))
            .await?
            .map_err(ContextError::from)
    }

    /// Sets the score for a particular agent returning an error if it already has a score
    pub async fn set_new_score(
        &self,
        agent: String,
        score: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::insert_new_score(conn, agent, score)
        })
        .await
    }

    /// Overwrites the score for a particular agent or inserts the score if no score currently exists.
    pub async fn upsert_score(
        &self,
        agent: String,
        score: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| doxa_db::action::leaderboard::upsert_score(conn, agent, score))
            .await
    }

    /// Adds delta to the score or sets the score to `default + delta` if no score currently
    /// exists.
    pub async fn update_score(
        &self,
        agent: String,
        delta: i32,
        default: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::update_score(conn, agent, delta, default)
        })
        .await
    }

    /// Gets the agent's current score if it exists.
    pub async fn get_agent_score(&self, agent: String) -> Result<Option<i32>, ContextError> {
        self.run_query(move |conn| doxa_db::action::leaderboard::get_score(conn, agent))
            .await
            .map(|res| res.map(|s| s.score))
    }

    /// Gets the highest achieving agent and it's score
    pub async fn get_high_score(
        &self,
        user_id: i32,
    ) -> Result<Option<LeaderboardScore>, ContextError> {
        self.run_query(move |conn| doxa_db::action::leaderboard::get_user_high_score(conn, user_id))
            .await
    }

    /// Get the **unordered** game participants
    pub async fn get_game_participants_unordered(
        &self,
        game_id: i32,
    ) -> Result<Vec<GameParticipantUser>, ContextError> {
        self.run_query(move |conn| doxa_db::action::game::get_game_participants(conn, game_id))
            .await
    }

    /// Get the list of agent IDs in the order of their agent IDs within the game (the same order
    /// as was specified in the match request).
    ///
    /// This relies on the start event, if this doesn't exist then this returns a `StartEventNotFound`
    /// error.
    pub async fn get_game_participants_ordered(
        &self,
        game_id: i32,
    ) -> Result<Vec<String>, ContextError> {
        let start_event = self
            .get_start_event(game_id)
            .await?
            .ok_or(StartEventNotFound { game_id })?;

        Ok(start_event.payload.agents)
    }

    pub(crate) async fn get_single_event_by_type(
        &self,
        game_id: i32,
        event_type: String,
    ) -> Result<Option<GameEvent<serde_json::Value>>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::get_single_game_event_by_event_type(conn, game_id, event_type)
                .map(|event| event.map(|event| event.into()))
        })
        .await
    }

    pub async fn get_start_event(
        &self,
        game_id: i32,
    ) -> Result<Option<GameEvent<StartEvent>>, ContextError> {
        let event = self
            .get_single_event_by_type(game_id, "_START".into())
            .await?;

        Ok(event
            .map(|event| event.try_map_payload(|payload| serde_json::from_value(payload)))
            .transpose()
            .map_err(|error| ParseSystemMessageError {
                event_type: "_START".into(),
                game_id,
                error,
            })?)
    }

    /// Returns a list of the games the agent has participated in, if the agent does not exist this
    /// will just return an empty vector.
    pub async fn get_agent_games(&self, agent: String) -> Result<Vec<Game>, ContextError> {
        self.run_query(move |conn| doxa_db::action::game::get_agent_games(conn, agent))
            .await
    }

    pub async fn get_agent(&self, agent: String) -> Result<Option<AgentUpload>, ContextError> {
        self.run_query(move |conn| doxa_db::action::storage::get_agent(conn, agent))
            .await
    }

    pub async fn get_user_agents(&self, user_id: i32) -> Result<Vec<AgentUpload>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::storage::list_agents(conn, user_id, competition_id)
        })
        .await
    }

    pub async fn get_active_agent(
        &self,
        user_id: i32,
    ) -> Result<Option<AgentUpload>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::storage::get_active_agent(conn, user_id, competition_id)
        })
        .await
    }

    pub async fn get_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<User>, ContextError> {
        self.run_query(move |conn| doxa_db::action::user::get_user_by_username(conn, &username))
            .await
    }

    /// Returns the list of active agents and their scores in descending order for this competition
    /// (only for those that exist in the leaderboard since an agent could be active but not yet on the leaderboard)
    pub async fn get_leaderboard(
        &self,
    ) -> Result<Vec<(AgentUpload, LeaderboardScore)>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::active_leaderboard(conn, competition_id)
        })
        .await
    }

    pub async fn add_game_result(
        &self,
        agent: String,
        game: i32,
        result: i32,
    ) -> Result<GameResult, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::add_game_result(
                conn,
                &GameResult {
                    agent,
                    game,
                    result,
                },
            )
        })
        .await
    }

    /// Inserts a group of game results at once only if all the agents are currently active.
    /// This guarantees (using a transaction) that if the rows are inserted the agents were all active at the time of insertion.
    /// If any agent in the group was not active then this will not return an error but it will also not insert into the DB.
    /// If `update_score_by_sum` is true then this will sum the game results and set that to the score as part of the same transaction.
    pub async fn add_game_results_active(
        &self,
        game_id: i32,
        results: impl Iterator<Item = (String, i32)> + Send + 'static,
        update_score_by_sum: bool,
    ) -> Result<(), ContextError> {
        if let Err(e) = self
            .run_query(move |conn| {
                conn.build_transaction().repeatable_read().run(|| {
                    for (agent, result) in results {
                        let agent =
                            doxa_db::action::storage::get_agent_required(conn, agent.clone())?;

                        if !agent.active {
                            return Err(DieselError::RollbackTransaction);
                        }

                        doxa_db::action::game::add_game_result(
                            conn,
                            &GameResult {
                                agent: agent.id.clone(),
                                game: game_id,
                                result,
                            },
                        )?;

                        if update_score_by_sum {
                            let score =
                                doxa_db::action::game::sum_game_results(conn, agent.id.clone())?
                                    .unwrap_or(0);
                            doxa_db::action::leaderboard::upsert_score(
                                conn,
                                agent.id,
                                score as i32,
                            )?;
                        }
                    }

                    Ok(())
                })
            })
            .await
        {
            if matches!(e, ContextError::Diesel(DieselError::RollbackTransaction)) {
                debug!(%game_id, "rolled back game results transaction due to inactive agent");
                return Ok(());
            }

            Err(e)
        } else {
            Ok(())
        }
    }

    pub async fn sum_game_results(&self, agent: String) -> Result<Option<i64>, ContextError> {
        self.run_query(move |conn| doxa_db::action::game::sum_game_results(conn, agent))
            .await
    }

    /// For each game that the given agent was a participant in, this will remove the game results for each of those games (including results for agents other than the current one).
    /// This will then return the results that is removed.
    /// This list is likely useful to then go through and update scores
    pub async fn remove_game_result_by_participant(
        &self,
        agent: String,
    ) -> Result<Vec<GameResult>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::remove_game_result_by_participant(conn, agent)
        })
        .await
    }

    /// Sums the game_result for a particular agent and then sets the agent's score to that value.
    /// If there are no game results for that agent it will set the score to 0
    pub async fn set_score_by_game_result_sum(
        &self,
        agent: String,
    ) -> Result<LeaderboardScore, ContextError> {
        // TODO: use transaction
        let score = self.sum_game_results(agent.clone()).await?.unwrap_or(0);
        self.upsert_score(agent, score as i32).await
    }

    pub async fn remove_game_result_by_participant_and_update_scores_by_sum(
        &self,
        agent: String,
    ) -> Result<(), ContextError> {
        // TODO: use transaction?
        // Maybe only update scores if the agent is active?
        let mut unique_agents = HashSet::new();
        let results = self
            .remove_game_result_by_participant(agent.clone())
            .await?;

        for result in results {
            if result.agent != agent {
                unique_agents.insert(result.agent);
            }
        }

        for agent in unique_agents.into_iter() {
            self.set_score_by_game_result_sum(agent).await?;
        }

        Ok(())
    }
}
