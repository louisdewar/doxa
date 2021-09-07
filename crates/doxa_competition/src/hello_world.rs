use std::convert::Infallible;

use crate::{
    client::{Competition, Context},
    error::ContextError,
};
use async_trait::async_trait;
use doxa_executor::{client::GameClient, context::GameContext};
use doxa_mq::model::UploadEvent;

use serde::{Deserialize, Serialize};

/// A dummy competition for development/debugging
pub struct HelloWorldCompetiton;

#[async_trait]
impl Competition for HelloWorldCompetiton {
    type GameClient = HelloWorldGameClient;

    const COMPETITION_NAME: &'static str = "helloworld";

    async fn startup(&self, _context: &mut Context<Self>) -> Result<(), ContextError> {
        println!("[hello_world] starting up");

        Ok(())
    }

    fn configure_routes(&self, _service: &mut doxa_core::actix_web::web::ServiceConfig) {
        println!("[hello_world] configuring routes");
    }

    async fn on_upload(
        &self,
        context: &mut Context<Self>,
        upload_event: UploadEvent,
    ) -> Result<(), ContextError> {
        println!("[hello_world] on_upload - agent {}", upload_event.agent);
        context
            .emit_match_request(vec![upload_event.agent], ())
            .await?;

        Ok(())
    }

    async fn on_game_event(
        &self,
        _context: &mut Context<Self>,
        event: HelloWorldGameEvent,
    ) -> Result<(), ContextError> {
        println!("Received game event: {:?}", event);

        Ok(())
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
}

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
    },
}

#[async_trait]
impl GameClient for HelloWorldGameClient {
    type Error = Infallible;
    type MatchRequest = ();
    type GameEvent = HelloWorldGameEvent;

    async fn run<'a>(
        _match_request: Self::MatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), doxa_executor::error::GameError<Self::Error>> {
        context.expect_n_agents(1)?;

        let message = context.next_message(0).await?;
        println!(
            "Got message from agent {}",
            String::from_utf8_lossy(message)
        );

        context
            .send_message_to_agent(0, b"PLEASE ECHO THIS MESSAGE\n")
            .await?;

        let expected_output = b"echo PLEASE ECHO THIS MESSAGE";

        let message = context.next_message(0).await?;
        if message == expected_output {
            println!("Agent responded sucessfully ✅");
            context
                .emit_game_event(HelloWorldGameEvent::RespondedSuccessfully {
                    output: String::from_utf8_lossy(expected_output).to_string(),
                })
                .await?;
        } else {
            let agent_output = String::from_utf8_lossy(message).to_string();
            println!("Agent responded incorrectly ❌ ({})", agent_output);
            context
                .emit_game_event(HelloWorldGameEvent::RespondedIncorrectly {
                    expected_output: String::from_utf8_lossy(expected_output).to_string(),
                    agent_output,
                })
                .await?;
        }

        Ok(())
    }
}
