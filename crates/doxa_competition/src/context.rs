use std::{marker::PhantomData, ops::Deref, sync::Arc};

use doxa_core::{chrono::Utc, tokio};
use doxa_db::{
    diesel::PgConnection,
    model::game::{Game, GameParticipant, InsertableGame},
    DieselError, PgPool,
};
use doxa_executor::client::GameClient;
use doxa_mq::{model::MatchRequest, MQPool};

use crate::{client::Competition, error::ContextError};

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

impl<C: Competition> Context<C> {
    pub(crate) fn new(mq_pool: Arc<MQPool>, pg_pool: Arc<PgPool>, competition_id: i32) -> Self {
        Context {
            mq_pool,
            pg_pool,
            competition: PhantomData,
            competition_id,
        }
    }
}

impl<C: Competition> Context<C> {
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
        let game = tokio::task::spawn_blocking::<_, Result<Game, DieselError>>({
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

    /// TODO: create a nice DB abstraction for competitions
    pub async fn db_connection(&self) -> Result<impl Deref<Target = PgConnection>, ContextError> {
        let pool = self.pg_pool.clone();
        let connection = tokio::task::spawn_blocking(move || pool.get()).await??;

        Ok(connection)
    }
}
