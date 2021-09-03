use std::time::Duration;

use doxa_core::{lapin::Channel, tokio};
use doxa_mq::model::GameEvent;
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::timeout;

use crate::{agent::Agent, error::GameContextError};

pub const MAX_MESSAGE_TIME: Duration = Duration::from_secs(120);

pub struct GameContext<'a> {
    agents: &'a mut Vec<Agent>,
    event_queue_name: &'a str,
    event_channel: &'a Channel,
}

impl<'a> GameContext<'a> {
    pub(crate) fn new(
        agents: &'a mut Vec<Agent>,
        event_queue_name: &'a str,
        event_channel: &'a Channel,
    ) -> Self {
        GameContext {
            agents,
            event_queue_name,
            event_channel,
        }
    }

    pub async fn emit_event<T: Serialize>(
        &mut self,
        event: &GameEvent<T>,
    ) -> Result<(), GameContextError> {
        doxa_mq::action::publish(
            self.event_channel,
            self.event_queue_name,
            doxa_mq::action::serialize(event).unwrap(),
        )
        .await
        .map_err(|e| GameContextError::Emit(e))
        .map(|_| ())
    }

    pub fn deserialize_match_request<T: DeserializeOwned>(
        &self,
        payload: &[u8],
    ) -> Result<T, GameContextError> {
        doxa_mq::action::deserialize(payload).map_err(|e| GameContextError::PayloadDeserialize(e))
    }

    /// Returns the number of agents playing in the game.
    /// Agents have IDs from 0 up to but not including `self.agents()`.
    pub fn agents(&self) -> usize {
        self.agents.len()
    }

    fn agent_mut(&mut self, agent_id: usize) -> Result<&mut Agent, GameContextError> {
        if agent_id >= self.agents() {
            return Err(GameContextError::UnknownAgent {
                id: agent_id,
                max: self.agents() - 1,
            });
        }

        Ok(&mut self.agents[agent_id])
    }

    /// Gets the next message from a particular agent.
    /// This method is cancel safe.
    pub async fn next_message(&mut self, agent_id: usize) -> Result<&[u8], GameContextError> {
        let agent = self.agent_mut(agent_id)?;

        let msg = timeout(MAX_MESSAGE_TIME, agent.next_message())
            .await
            .map_err(|_| GameContextError::TimeoutWaitingForMessage { agent_id })??;
        Ok(msg)
    }

    /// Sends a message to a particular agent's STDIN.
    /// This by default will NOT include a new line.
    /// The data will be sent to STDIN as is.
    pub async fn send_message_to_agent(
        &mut self,
        agent_id: usize,
        msg: &[u8],
    ) -> Result<(), GameContextError> {
        let agent = self.agent_mut(agent_id)?;

        agent
            .send_agent_input(msg)
            .await
            .map_err(|e| GameContextError::SendInput(e))?;

        Ok(())
    }
}
