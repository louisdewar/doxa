use std::{marker::PhantomData, sync::Arc};

use doxa_core::lapin::Channel;
use doxa_core::tracing::error;
use doxa_mq::model::MatchRequest;
use futures::{
    future::{join_all, try_join_all},
    TryFutureExt,
};

use crate::{
    agent::{VMAgent, VMAgentSettings},
    client::{ForfeitError, GameClient, GameError},
    context::GameContext,
    error::{AgentTerminated, GameContextError, GameManagerError},
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
        let additional_mounts = C::additional_mounts(&match_request.payload);

        let mut mounts = settings.base_mounts.clone();
        mounts.extend(additional_mounts);

        let vm_agent_settings = VMAgentSettings {
            agent_ram_mb: C::AGENT_RAM,
            scratch_size_mb: C::AGENT_SCRATCH,
            mounts,
        };

        let agents = match_request.agents.into_iter().map(|agent_id| {
            VMAgent::new(
                competition_name,
                agent_id,
                &settings.agent_retrieval,
                &settings,
                vm_agent_settings.clone(),
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
            Err(mut error) => {
                let vm_logs = if let Some(agent_id) = error.forfeit() {
                    let stderr = match &mut error {
                        GameError::Context(GameContextError::AgentTerminated(
                            AgentTerminated { stderr, .. },
                            // The stderr isn't needed again
                        )) => stderr.take(),
                        _ => None,
                    };
                    context
                        .forfeit_agent(agent_id, stderr, error.forfeit_message())
                        .await?;

                    (0..context.agents()).map(|_| None).collect()
                } else {
                    let mut agents = Vec::new();
                    // Take ownership of agents swapping it with an empty vector (they aren't
                    // needed anymore)
                    std::mem::swap(&mut agents, context.agents);
                    // TODO: in future if we can figure out which VM the error came from we should
                    // only include those logs and put None elsewhere.
                    let vm_logs = join_all(agents.into_iter().enumerate().map(|(i, agent)| {
                        agent.shutdown().map_ok_or_else(move |e| {
                            error!(error=%e, debug=?e, agent_id=%i, "failed to shutdown agent and collect logs");
                            None
                        }, Some)
                    }))
                    .await;

                    vm_logs
                };

                context.emit_error_event(&error, vm_logs).await?;

                Err(error)
            }
        };

        context.emit_end_event().await?;

        res
    }
}
