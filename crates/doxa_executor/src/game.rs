use std::{marker::PhantomData, sync::Arc};

use doxa_core::lapin::Channel;
use doxa_mq::model::MatchRequest;
use futures::future::try_join_all;

use crate::{
    agent::VMAgent,
    client::{ForfeitError, GameClient, GameError},
    context::GameContext,
    error::GameManagerError,
    Settings,
};

pub struct GameManager<C: GameClient> {
    client: PhantomData<C>,
    agents: Vec<VMAgent>,
    event_queue_name: String,
    event_channel: Channel,
    client_match_request: C::MatchRequest,
    game_id: i32,
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
            VMAgent::new(
                competition_name,
                C::AGENT_RAM,
                agent_id,
                &settings.agent_retrieval,
                &settings,
            )
        });

        let agents = try_join_all(agents)
            .await
            .map_err(GameManagerError::StartAgent)?;

        Ok(GameManager {
            agents,
            client: PhantomData,
            event_queue_name,
            event_channel,
            client_match_request: match_request.payload,
            game_id: match_request.game_id,
        })
    }

    /// Runs the game to completion
    pub async fn run(mut self) -> Result<(), GameError<C::Error>> {
        let mut context = GameContext::new(
            &mut self.agents,
            &self.event_queue_name,
            &self.event_channel,
            self.game_id,
        );

        context.emit_start_event().await?;

        let res = match C::run(self.client_match_request, &mut context).await {
            Ok(()) => Ok(()),
            Err(error) => {
                if let Some(agent_id) = error.forfeit() {
                    context.forfeit_agent(agent_id).await?;
                }

                context.emit_error_event(&error).await?;

                Err(error)
            }
        };

        context.emit_end_event().await?;

        res
    }
}
