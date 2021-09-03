use std::{convert::Infallible, error::Error};

use crate::{client::Competition, error::ContextError};
use async_trait::async_trait;
use doxa_executor::{client::GameClient, context::GameContext};
use doxa_mq::model::{MatchRequest, UploadEvent};

/// A dummy competition for development/debugging
pub struct HelloWorldCompetiton;

#[async_trait]
impl Competition for HelloWorldCompetiton {
    type GameClient = HelloWorldGameClient;

    const COMPETITION_NAME: &'static str = "helloworld";

    async fn startup(
        &self,
        _context: &mut crate::client::Context<Self>,
    ) -> Result<(), ContextError> {
        println!("[hello_world] starting up");

        Ok(())
    }

    fn configure_routes(&self, _service: &mut doxa_core::actix_web::web::ServiceConfig) {
        println!("[hello_world] configuring routes");
    }

    async fn on_upload(
        &self,
        context: &mut crate::client::Context<Self>,
        upload_event: UploadEvent,
    ) -> Result<(), ContextError> {
        println!("[hello_world] on_upload - agent {}", upload_event.agent);
        context
            .emit_match_request(vec![upload_event.agent], ())
            .await?;

        Ok(())
    }

    async fn on_execution_result(
        &self,
        _context: &mut crate::client::Context<Self>,
    ) -> Result<(), ContextError> {
        println!("[hello_world] on_execution_result");

        Ok(())
    }
}

pub struct HelloWorldGameClient;

#[async_trait]
impl GameClient for HelloWorldGameClient {
    type Error = Infallible;
    type MatchRequest = ();

    async fn run<'a>(
        _match_request: Self::MatchRequest,
        context: &mut GameContext<'a>,
    ) -> Result<(), doxa_executor::error::GameError<Self::Error>> {
        let message = context.next_message(0).await?;
        println!(
            "Got message from agent {}",
            String::from_utf8_lossy(message)
        );

        Ok(())
    }
}
