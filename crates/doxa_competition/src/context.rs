use std::{marker::PhantomData, ops::Deref, sync::Arc};

use doxa_core::tokio;
use doxa_db::{diesel::PgConnection, PgPool};
use doxa_executor::client::GameClient;
use doxa_mq::{model::MatchRequest, MQPool};
use serde::Serialize;

use crate::{
    client::{self, Competition},
    error::ContextError,
};

// Maybe rename to BaseContext which contains stuff that can be cloned around then before actually
// passing it into things such as routes we extract the db_pool and store it in the Context, or we
// make those kinds of methods take in a DbConnection and provide another method which takes in
// DbPool and returns DbConnection
//
// Maybe make context generic over the competition to allow for automatically having the correct
// type for match request. Build one context per competiton
#[derive(Clone)]
pub struct Context<C: Competition + ?Sized> {
    mq_pool: Arc<MQPool>,
    pg_pool: Arc<PgPool>,
    competition: PhantomData<C>,
}

impl<C: Competition> Context<C> {
    pub(crate) fn new(mq_pool: Arc<MQPool>, pg_pool: Arc<PgPool>) -> Self {
        Context {
            mq_pool,
            pg_pool,
            competition: PhantomData,
        }
    }
}

impl<C: Competition> Context<C> {
    /// Emits a match request event.
    ///
    /// The `GameClient` will recieve the match_request on initialization.
    pub async fn emit_match_request(
        &self,
        agents: Vec<String>,
        match_request: <C::GameClient as GameClient>::MatchRequest,
    ) -> Result<(), ContextError> {
        let connection = self.mq_pool.get().await?;

        let match_request = MatchRequest {
            agents,
            payload: match_request,
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

    // pub fn mongo(&self) {
    //     todo!();
    // }

    // pub fn register_timer<A: Fn(&mut Context) -> B, B: Future<Output = ()>>(
    //     &self,
    //     duration: Duration,
    // ) {
    //     todo!()
    // }
}
