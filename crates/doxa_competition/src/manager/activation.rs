use std::sync::Arc;

use doxa_core::{
    lapin::{message::Delivery, options::BasicAckOptions},
    tokio,
    tracing::{error, info, span, Level},
    tracing_futures::Instrument,
};
use doxa_db::model::storage::AgentUpload;
use doxa_mq::model::ActivationEvent;

use crate::{error::ContextError, Settings};

use futures::{StreamExt, TryFutureExt};

use crate::client::{Competition, Context};

impl<C: Competition> Context<C> {
    /// Sets the activation flag for the given `agent_id` to true.
    /// If another agent currently has the activation flag set to true for this user and
    /// competition it will unset it (for that agent) and return that agent - the deactivated
    /// agent.
    ///
    /// If the agent does not exist the transaction will be rolled back and an error will be
    /// returned (no data will be changed).
    ///
    /// This action is performed atomically.
    async fn activate_agent(&self, agent_id: String) -> Result<Option<AgentUpload>, ContextError> {
        self.run_query(move |conn| {
            conn.build_transaction().repeatable_read().run(move || {
                let agent = doxa_db::action::storage::get_agent_required(conn, agent_id.clone())?;
                let deactivated_agent = doxa_db::action::storage::deactivate_agent(
                    conn,
                    agent.competition,
                    agent.owner,
                )?;

                doxa_db::action::storage::activate_agent(conn, agent_id)?;

                Ok(deactivated_agent)
            })
        })
        .await
        .map_err(|e| e)
    }

    /// Deactivates the agent if it exists and if it is currently activated.
    /// If either of these preconditions are false then `Ok(None)` is returned
    async fn deactivate_agent(
        &self,
        agent_id: String,
    ) -> Result<Option<AgentUpload>, ContextError> {
        self.run_query(move |conn| doxa_db::action::storage::deactivate_agent_by_id(conn, agent_id))
            .await
            .map_err(|e| e)
    }
}

#[derive(Clone)]
pub(super) struct AgentActivationManager<C: Competition> {
    settings: Arc<Settings>,
    competition: Arc<C>,
    context: Arc<Context<C>>,
}

impl<C: Competition> AgentActivationManager<C> {
    pub fn new(settings: Arc<Settings>, competition: Arc<C>, context: Arc<Context<C>>) -> Self {
        AgentActivationManager {
            settings,
            competition,
            context,
        }
    }

    /// Activates the agent and then calls the `on_agent_activated` event if it is successful.
    /// If this required deactivating an agent then the `on_agent_deactivated` will be called
    /// first.
    ///
    /// This will first check to make sure the agent has not been deleted (e.g. by a subsequent
    /// upload).
    /// In this case the method silently does nothing.
    async fn activate_agent(&self, agent_id: String) -> Result<(), ContextError> {
        // This isn't atomic, but this isn't a big deal since it's okay to delete an active agent
        // (it just means it won't play future) matches.
        // The only utility of this is to prevent wasting time queueing matches we know are going
        // to be skipped because the agent doesn't exist.
        let agent = self.context.get_agent_required(agent_id.clone()).await?;

        if agent.deleted {
            info!(%agent_id, "not activating agent because it was deleted");
            return Ok(());
        }

        if let Some(deactivated_agent) = self.context.activate_agent(agent_id.clone()).await? {
            let span = span!(Level::INFO, "deactiving agent before activating new one", old_agent = %deactivated_agent.id);
            self.competition
                .on_agent_deactivated(&self.context, deactivated_agent.id)
                .instrument(span)
                .await
                .map_err(|error| {
                    error!(
                        %error,
                        error_debug = ?error,
                        "on_agent_deactivated failed during the process of activating another agent"
                    );
                    error
                })?;
        }

        self.competition
            .on_agent_activated(&self.context, agent_id)
            .await?;

        Ok(())
    }

    /// Both deactivates the agent and calls the deactiate handler.
    /// If the agent doesn't exist or has already been deactivated this will not do anything.
    async fn deactivate_agent(&self, agent_id: String) -> Result<(), ContextError> {
        if let Some(agent) = self.context.deactivate_agent(agent_id).await? {
            self.competition
                .on_agent_deactivated(&self.context, agent.id)
                .await?;
        }

        Ok(())
    }

    async fn handle_activation_event(
        &self,
        delivery: Delivery,
        event: ActivationEvent,
    ) -> Result<(), ContextError> {
        if event.activating {
            self.activate_agent(event.agent).await?;
        } else {
            self.deactivate_agent(event.agent).await?;
        }

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to acknowledge MQ");

        Ok(())
    }

    pub async fn start(self) {
        let connection = self
            .settings
            .mq_pool
            .get()
            .await
            .expect("Failed to get MQ connection");

        let mut consumer =
            doxa_mq::action::get_activation_event_consumer(&connection, C::COMPETITION_NAME)
                .await
                .unwrap();

        let span = span!(
            Level::INFO,
            "agent activation event listener",
            competition = C::COMPETITION_NAME
        );

        let future = async move {
            while let Some(message) = consumer.next().await {
                let (_, delivery) = message.expect("Error connecting to MQ");

                let event: ActivationEvent = doxa_mq::action::deserialize(&delivery.data)
                    .expect("Improperly formatted message");
                let agent_id = event.agent.clone();

                let span = span!(
                    Level::INFO,
                    "handle agent activation request",
                    %agent_id,
                    %event.activating
                );

                let _: Result<(), ()> = self
                    .handle_activation_event(delivery, event)
                    .map_err(|error| {
                        error!(%error, error_debug = ?error, "failed to handle activation request");
                    })
                    .instrument(span)
                    .await;
            }
        };

        tokio::spawn(future.instrument(span));
    }
}
