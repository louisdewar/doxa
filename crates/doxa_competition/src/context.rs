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
    model::{ActivationEvent, GameEvent, MatchRequest},
    MQPool,
};

use crate::{
    client::Competition,
    error::{AgentNotActive, AgentNotFound, ContextError, ParseSystemMessageError},
};

// TODO: consider moving context methods in their own folders, this file is getting a bit unwieldy

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
                        queued_at: Utc::now(),
                        competition,
                    },
                )?;

                for (index, agent) in agents.into_iter().enumerate() {
                    doxa_db::action::game::add_participant(
                        &db,
                        &GameParticipant {
                            index: index as i32,
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

        let activated_at = agent.activated_at.ok_or(AgentNotActive)?;

        let active_agents = self
            .run_query(move |conn| {
                doxa_db::action::storage::get_active_agents_activated_before(
                    conn,
                    agent.competition,
                    activated_at,
                )
            })
            .await?;

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
    ///
    /// The key can be set if there are multiple leaderboards that you want to differentiate
    /// between.
    /// If no key is provided (i.e. `None`) then the `primary` leaderboard is used.
    pub async fn set_new_score(
        &self,
        key: Option<String>,
        agent: String,
        score: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::insert_new_score(conn, key, agent, score)
        })
        .await
    }

    /// Overwrites the score for a particular agent or inserts the score if no score currently exists.
    pub async fn upsert_score(
        &self,
        key: Option<String>,
        agent: String,
        score: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::upsert_score(conn, key, agent, score)
        })
        .await
    }

    /// Adds delta to the score or sets the score to `default + delta` if no score currently
    /// exists.
    pub async fn update_score(
        &self,
        key: Option<String>,
        agent: String,
        delta: i32,
        default: i32,
    ) -> Result<LeaderboardScore, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::update_score(conn, key, agent, delta, default)
        })
        .await
    }

    /// Gets the agent's current score if it exists.
    pub async fn get_agent_score(
        &self,
        agent: String,
        key: Option<String>,
    ) -> Result<Option<i32>, ContextError> {
        self.run_query(move |conn| doxa_db::action::leaderboard::get_score(conn, key, agent))
            .await
            .map(|res| res.map(|s| s.score))
    }

    /// Gets the highest achieving agent and it's score
    pub async fn get_high_score(
        &self,
        user_id: i32,
        key: Option<String>,
    ) -> Result<Option<LeaderboardScore>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::get_user_high_score(conn, user_id, competition_id, key)
        })
        .await
    }

    /// Get the **unordered** game participants
    pub async fn get_game_participants_unordered(
        &self,
        game_id: i32,
    ) -> Result<Vec<GameParticipantUser>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::get_game_participants_unordered(conn, game_id)
        })
        .await
    }

    /// Get the list of agent IDs and their owner in the order of their agent IDs within the game (the same order
    /// as was specified in the match request).
    pub async fn get_game_participants_ordered(
        &self,
        game_id: i32,
    ) -> Result<Vec<(String, User)>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::get_game_participants_ordered(conn, game_id)
        })
        .await
    }

    /// Gets the user that is the owner of the agent.
    /// This assumes that the agent is supposed to exist, if it doesn't it will return a DieselError which translates to an internal server error.
    pub async fn get_agent_owner(&self, agent_id: String) -> Result<User, ContextError> {
        self.run_query(move |conn| doxa_db::action::storage::get_agent_owner(conn, agent_id))
            .await
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, ContextError> {
        self.run_query(move |conn| doxa_db::action::user::get_user_by_id(conn, user_id))
            .await
    }

    pub async fn get_events(
        &self,
        game_id: i32,
    ) -> Result<Vec<doxa_db::model::game::GameEvent>, ContextError> {
        self.run_query(move |conn| doxa_db::action::game::get_game_events(conn, game_id))
            .await
    }

    pub async fn get_game_events_by_event_type(
        &self,
        game_id: i32,
        event_type: String,
    ) -> Result<Vec<doxa_db::model::game::GameEvent>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::get_game_events_by_event_type(conn, game_id, event_type)
        })
        .await
    }

    /// If there are more than one events with this event_type in this game, which event is
    /// returned is undefined.
    pub async fn get_single_event_by_type(
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
            .map(|event| event.try_map_payload(serde_json::from_value))
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

    // Gets an agent returning an error if it doesn't exist
    pub async fn get_agent_required(&self, agent: String) -> Result<AgentUpload, ContextError> {
        self.run_query(move |conn| doxa_db::action::storage::get_agent_required(conn, agent))
            .await
    }

    pub async fn is_agent_active(&self, agent: String) -> Result<bool, ContextError> {
        let agent = self.get_agent(agent).await?.ok_or(AgentNotFound)?;

        Ok(agent.active)
    }

    pub async fn get_user_agents(&self, user_id: i32) -> Result<Vec<AgentUpload>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::storage::list_agents(conn, user_id, competition_id)
        })
        .await
    }

    /// Gets a list of games which the user has participated where ALL of the agents involved are active
    /// This is ordered by game start time ascending.
    pub async fn get_user_active_games(&self, user_id: i32) -> Result<Vec<Game>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::game::get_user_active_games(conn, user_id, competition_id)
        })
        .await
    }

    pub async fn get_game_result(
        &self,
        game_id: i32,
        agent_id: String,
    ) -> Result<Option<GameResult>, ContextError> {
        self.run_query(move |conn| doxa_db::action::game::get_game_result(conn, game_id, agent_id))
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

    pub async fn get_game_by_id(&self, game_id: i32) -> Result<Option<Game>, ContextError> {
        self.run_query(move |conn| {
            doxa_db::action::game::get_game_by_id(conn, game_id, C::COMPETITION_NAME)
        })
        .await
    }

    /// Returns the list of active agents and their scores in descending order for this competition
    /// (only for those that exist in the leaderboard since an agent could be active but not yet on the leaderboard)
    pub async fn get_leaderboard(
        &self,
        key: Option<String>,
    ) -> Result<Vec<(User, LeaderboardScore)>, ContextError> {
        let competition_id = self.competition_id;
        self.run_query(move |conn| {
            doxa_db::action::leaderboard::active_leaderboard(conn, competition_id, key)
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

    /// Adds the game result to the database only if the game is **not outdated**.
    pub async fn add_game_result_active(
        &self,
        agent: String,
        game: i32,
        result: i32,
    ) -> Result<(), ContextError> {
        self.run_query(move |conn| {
            // TODO: use an INSERT from SELECT query to avoid the transaction
            Ok(conn.build_transaction().repeatable_read().run(|| {
                let game = doxa_db::action::game::get_game_by_id_required(
                    conn,
                    game,
                    C::COMPETITION_NAME,
                )?;

                if game.outdated {
                    return Ok(());
                }

                doxa_db::action::game::add_game_result(
                    conn,
                    &GameResult {
                        agent,
                        game: game.id,
                        result,
                    },
                )?;

                Ok(())
            }))
        })
        .await?
    }

    pub async fn get_user_rank(
        &self,
        user_id: i32,
    ) -> Result<Option<(i32, LeaderboardScore)>, ContextError> {
        self.run_query(move |conn| doxa_db::action::leaderboard::get_user_rank(conn, user_id))
            .await
    }

    /// Inserts a group of game results at once only if the game is not outdated.
    /// This guarantees (using a transaction) that if the rows are inserted the agents were all active at the time of insertion.
    /// If any agent in the group was not active then this will not return an error but it will also not insert into the DB.
    /// If `update_score_by_sum` is true then this will sum the game results and set that to the score as part of the same transaction.
    pub async fn add_game_results_active(
        &self,
        key: Option<String>,
        game_id: i32,
        results: impl Iterator<Item = (String, i32)> + Send + 'static,
        update_score_by_sum: bool,
    ) -> Result<(), ContextError> {
        // TODO: re-write this code properly using the new outdated field on games (should make
        // things simpler)
        if let Err(e) = self
            .run_query(move |conn| {
                conn.build_transaction().repeatable_read().run(|| {
                    let game = doxa_db::action::game::get_game_by_id_required(
                        conn,
                        game_id,
                        C::COMPETITION_NAME,
                    )?;

                    if game.outdated {
                        return Ok(());
                    }

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
                            let score = doxa_db::action::game::sum_non_outdated_game_results(
                                conn,
                                agent.id.clone(),
                            )?
                            .unwrap_or(0);
                            doxa_db::action::leaderboard::upsert_score(
                                conn,
                                key.clone(),
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
        self.run_query(move |conn| {
            doxa_db::action::game::sum_non_outdated_game_results(conn, agent)
        })
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
        key: Option<String>,
        agent: String,
    ) -> Result<LeaderboardScore, ContextError> {
        // TODO: use transaction
        let score = self.sum_game_results(agent.clone()).await?.unwrap_or(0);
        self.upsert_score(key, agent, score as i32).await
    }

    pub async fn remove_game_result_by_participant_and_update_scores_by_sum(
        &self,
        key: Option<String>,
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
            self.set_score_by_game_result_sum(key.clone(), agent)
                .await?;
        }

        Ok(())
    }

    /// Adds the agent to the activation queue (this will automatically deactivate the previous
    /// agent before activating the new one including if the previous agent was this agent).
    pub async fn activate_agent(&self, agent: String) -> Result<(), ContextError> {
        let connection = self.mq_pool.get().await?;
        doxa_mq::action::emit_activation_event(
            &connection,
            &ActivationEvent {
                agent,
                activating: true,
                competition: C::COMPETITION_NAME.to_string(),
            },
        )
        .await?;

        Ok(())
    }

    /// Adds the agent to the activation queue (this will automatically deactivate the previous
    /// agent before activating the new one including if the previous agent was this agent).
    pub async fn deactivate_agent(&self, agent: String) -> Result<(), ContextError> {
        let connection = self.mq_pool.get().await?;
        doxa_mq::action::emit_activation_event(
            &connection,
            &ActivationEvent {
                agent,
                activating: false,
                competition: C::COMPETITION_NAME.to_string(),
            },
        )
        .await?;

        Ok(())
    }
}
