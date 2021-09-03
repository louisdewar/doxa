use std::{future::Future, marker::PhantomData, sync::Arc};

use doxa_core::lapin::Channel;
use doxa_mq::model::MatchRequest;
use futures::future::try_join_all;

use crate::{
    agent::Agent, client::GameClient, context::GameContext, error::GameManagerError, Settings,
};

pub struct GameManager<C: GameClient> {
    client: PhantomData<C>,
    agents: Vec<Agent>,
    event_queue_name: String,
    event_channel: Channel,
    client_match_request: C::MatchRequest,
}

impl<C: GameClient> GameManager<C> {
    pub async fn new(
        settings: Arc<Settings>,
        event_channel: Channel,
        event_queue_name: String,
        competition_name: &'static str,
        match_request: MatchRequest<C::MatchRequest>,
    ) -> Result<Self, GameManagerError<C::Error>> {
        let agents = match_request.agents.into_iter().map(|agent_id| {
            Agent::new(
                competition_name,
                agent_id,
                &settings.agent_retrieval,
                &settings,
            )
        });

        let agents = try_join_all(agents)
            .await
            .map_err(|e| GameManagerError::StartAgent(e))?;

        Ok(GameManager {
            agents,
            client: PhantomData,
            event_queue_name,
            event_channel,
            client_match_request: match_request.payload,
        })
    }

    /// Runs the game to completion
    pub async fn run(mut self) -> Result<(), GameManagerError<C::Error>> {
        let mut context = GameContext::new(
            &mut self.agents,
            &self.event_queue_name,
            &self.event_channel,
        );

        C::run(self.client_match_request, &mut context)
            .await
            .map_err(|e| GameManagerError::Runtime(e))
    }
}
