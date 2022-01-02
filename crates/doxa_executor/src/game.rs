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
    context::{GameContext, GameEventContext},
    error::{AgentTerminated, GameContextError, GameManagerError},
    Settings,
};

pub struct GameManager<C: GameClient> {
    client: PhantomData<C>,
    agents: Vec<VMAgent>,
    client_match_request: C::MatchRequest,
    game_event_context: GameEventContext<C>,
}

impl<C: GameClient> GameManager<C> {
    pub async fn new(
        settings: Arc<Settings>,
        event_channel: Channel,
        event_queue_name: String,
        competition_name: &'static str,
        match_request: MatchRequest<C::MatchRequest>,
    ) -> Result<Self, GameManagerError<C::Error>> {
        let mut game_event_context =
            GameEventContext::new(event_channel, event_queue_name, match_request.game_id);

        game_event_context
            .emit_start_event(match_request.agents.clone())
            .await
            .map_err(GameManagerError::EmitStartEvent)?;

        let additional_mounts = C::additional_mounts(&match_request.payload);

        let mut mounts = settings.base_mounts.clone();
        mounts.extend(additional_mounts);

        let vm_agent_settings = VMAgentSettings {
            agent_ram_mb: C::AGENT_RAM,
            scratch_size_mb: C::AGENT_SCRATCH,
            mounts,
        };

        let agents = match_request
            .agents
            .iter()
            .enumerate()
            .map(|(index, agent_id)| {
                VMAgent::new(
                    competition_name,
                    agent_id.clone(),
                    &settings.agent_retrieval,
                    &settings,
                    vm_agent_settings.clone(),
                )
                .map_err(move |e| (index, e))
            });

        let agents = match try_join_all(agents).await {
            Ok(agents) => agents,
            Err((index, mut e)) => {
                let mut vm_logs = vec![None; match_request.agents.len()];

                if let Some(logs) = e.logs.take() {
                    match logs {
                        Ok(logs) => vm_logs[index] = Some(logs),
                        Err(vm_logs_error) => {
                            error!(logs_error=%vm_logs_error, debug=?vm_logs_error, "failed to get VM logs for agent while processing a startup error");
                        }
                    }
                }

                if let Err(e) = game_event_context.emit_error_event(&e, vm_logs).await {
                    // If there is an error here the VM logs are lost, but it probably doesn't
                    // matter as it's unlikely that the VM logs will be related to the game event
                    // emit error
                    error!(error=%e, debug=?e, "failed to emit error event containing VM logs while processing a startup error");
                }

                return Err(GameManagerError::StartAgent(e));
            }
        };

        Ok(GameManager {
            agents,
            client: PhantomData,
            game_event_context,
            client_match_request: match_request.payload,
        })
    }

    /// Runs the game to completion
    pub async fn run(mut self) -> Result<(), GameError<C::Error>> {
        let mut context = GameContext::new(&mut self.agents, &mut self.game_event_context);

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

                if let Err(e) = self
                    .game_event_context
                    .emit_error_event(&error, vm_logs)
                    .await
                {
                    error!(error=%e, debug=?e, "failed to emit error event while processing an error during the game");
                }

                Err(error)
            }
        };

        if let Err(e) = self.game_event_context.emit_end_event().await {
            error!(error=%e, debug=?e, "failed to emit end event");
        }

        res
    }
}
