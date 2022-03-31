use std::{convert::Infallible, time::Duration};

use crate::{
    client::{Competition, Context, GameEvent},
    error::ContextError,
};
use async_trait::async_trait;
use doxa_auth::limiter::{LimiterConfig, TokenBucket};
use doxa_db::model::storage::AgentUpload;
use doxa_executor::{
    client::{GameClient, VMBackend},
    context::GameContext,
};

use serde::{Deserialize, Serialize};

/// A dummy competition for development/debugging
pub struct HelloWorldCompetiton;

#[async_trait]
impl Competition for HelloWorldCompetiton {
    type GameClient = HelloWorldGameClient;

    const COMPETITION_NAME: &'static str = "helloworld";

    async fn startup(&self, _context: &Context<Self>) -> Result<(), ContextError> {
        println!("[hello_world] starting up");

        Ok(())
    }

    async fn on_agent_activated(
        &self,
        context: &Context<Self>,
        agent: AgentUpload,
    ) -> Result<(), ContextError> {
        context
            .emit_match_request(vec![agent.id], (), &agent.execution_environment)
            .await?;

        Ok(())
    }

    async fn on_agent_deactivated(
        &self,
        _context: &Context<Self>,
        _agent: AgentUpload,
    ) -> Result<(), ContextError> {
        Ok(())
    }

    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<HelloWorldGameEvent>,
    ) -> Result<(), ContextError> {
        let mut participants = context
            .get_game_participants_unordered(event.game_id)
            .await?;
        let participant = participants.remove(0);

        // Only update score if the agent is active (we may have received a game event after a
        // significant delay meaning a new agent is now active).
        if !context.is_agent_active(participant.agent.clone()).await? {
            return Ok(());
        }

        let score = match event.payload {
            HelloWorldGameEvent::RespondedIncorrectly { .. } => -1,
            HelloWorldGameEvent::RespondedSuccessfully { .. } => 1,
            HelloWorldGameEvent::NoDoneMessage { .. } => return Ok(()),
        };

        context
            .set_new_score(None, participant.agent, score)
            .await?;

        Ok(())
    }

    fn upload_limiter(&self, key: String) -> LimiterConfig {
        let mut limiter = LimiterConfig::new(key);
        limiter.add_limit(TokenBucket::new(Duration::from_secs(120), 20));

        limiter
    }

    fn event_filter(
        game_event: HelloWorldGameEvent,
        _is_admin: bool,
        agent: Option<usize>,
    ) -> Option<serde_json::Value> {
        // Only yield events if the client was part of the event
        if agent.is_some() {
            Some(serde_json::to_value(game_event).unwrap())
        } else {
            None
        }
    }

    fn build_game_client(&self) -> Self::GameClient {
        Self::GameClient::default()
    }
}

#[derive(Default)]
pub struct HelloWorldGameClient;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum HelloWorldGameEvent {
    RespondedSuccessfully {
        output: String,
    },
    RespondedIncorrectly {
        expected_output: String,
        agent_output: String,
        file_output: String,
    },
    NoDoneMessage {
        outputed: String,
    },
}

#[async_trait]
impl GameClient for HelloWorldGameClient {
    type Error = Infallible;
    type MatchRequest = ();
    type GameEvent = HelloWorldGameEvent;

    async fn run<'a, B: VMBackend>(
        &self,
        _match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self, B>,
    ) -> Result<(), doxa_executor::error::GameError<Self::Error>> {
        context.set_max_message_time(Some(Duration::from_secs(5)));
        context.expect_n_agents(1)?;

        // Agents are not booted by default so we call reboot here (with zero arguments) to startup
        // the agent inside the VM.
        context.reboot_agent(0, vec![]).await?;

        context
            .send_message_to_agent(0, b"PLEASE ECHO THIS MESSAGE\n")
            .await?;

        let expected_output = b"echo PLEASE ECHO THIS MESSAGE";

        // It needs to be in this order as the file might not exist until after the agent outputs a
        // message
        let message = context.next_message(0).await?.to_owned();
        let file = context.take_file(0, "/output/test.txt").await?;
        context.send_message_to_agent(0, b"taken file\n").await?;

        if message == expected_output && file == expected_output {
            context
                .emit_game_event(
                    HelloWorldGameEvent::RespondedSuccessfully {
                        output: String::from_utf8_lossy(expected_output).to_string(),
                    },
                    "game",
                )
                .await?;
        } else {
            let agent_output = String::from_utf8_lossy(&message).to_string();
            let file_output = String::from_utf8_lossy(&file).to_string();

            context
                .emit_game_event(
                    HelloWorldGameEvent::RespondedIncorrectly {
                        expected_output: String::from_utf8_lossy(expected_output).to_string(),
                        agent_output,
                        file_output,
                    },
                    "game",
                )
                .await?;
        }

        let final_message = context.next_message(0).await?.to_owned();

        if &final_message != b"done" {
            context
                .emit_game_event(
                    HelloWorldGameEvent::NoDoneMessage {
                        outputed: String::from_utf8_lossy(&final_message).to_string(),
                    },
                    "game",
                )
                .await?
        }

        Ok(())
    }
}
