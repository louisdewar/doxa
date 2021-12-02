use std::{marker::PhantomData, time::Duration};

use doxa_core::{chrono::Utc, lapin::Channel, tokio};
use doxa_mq::model::GameEvent;
use serde::Serialize;
use tokio::time::timeout;

use crate::{
    agent::VMAgent,
    client::{GameClient, GameError},
    error::GameContextError,
    event::{ErrorEvent, ForfeitEvent, StartEvent},
};

pub const DEFAULT_MAX_MESSAGE_TIME: Duration = Duration::from_secs(120);

pub struct GameContext<'a, C: GameClient + ?Sized> {
    agents: &'a mut Vec<VMAgent>,
    event_queue_name: &'a str,
    event_channel: &'a Channel,
    client: PhantomData<C>,
    event_id: u32,
    game_id: i32,
    max_message_time: Duration,
}

impl<'a, C: GameClient> GameContext<'a, C> {
    pub(crate) fn new(
        agents: &'a mut Vec<VMAgent>,
        event_queue_name: &'a str,
        event_channel: &'a Channel,
        game_id: i32,
    ) -> Self {
        GameContext {
            agents,
            event_queue_name,
            event_channel,
            client: PhantomData,
            event_id: 0,
            game_id,
            max_message_time: DEFAULT_MAX_MESSAGE_TIME,
        }
    }

    /// The EVENT_TYPE must be a non-zero length string and cannot begin with an underscore.
    /// If you do not need to distinguish between event_types then just use `game` as a
    /// convention.
    pub async fn emit_game_event<S: Into<String>>(
        &mut self,
        event: C::GameEvent,
        event_type: S,
    ) -> Result<(), GameContextError> {
        self.emit_event_raw(event, event_type.into()).await
    }

    async fn emit_event_raw<T: Serialize>(
        &mut self,
        payload: T,
        event_type: String,
    ) -> Result<(), GameContextError> {
        let timestamp = Utc::now();
        let game_event = GameEvent {
            event_id: self.event_id,
            timestamp,
            event_type,
            payload,
            game_id: self.game_id,
        };
        self.event_id += 1;

        doxa_mq::action::publish(
            self.event_channel,
            self.event_queue_name,
            serde_json::to_vec(&game_event).unwrap(),
        )
        .await
        .map_err(GameContextError::Emit)
        .map(|_| ())
    }

    pub(crate) async fn emit_start_event(&mut self) -> Result<(), GameContextError> {
        self.emit_event_raw(
            StartEvent {
                agents: self
                    .agents
                    .iter()
                    .map(|agent| agent.id().to_string())
                    .collect(),
            },
            "_START".to_string(),
        )
        .await
    }

    pub(crate) async fn emit_end_event(&mut self) -> Result<(), GameContextError> {
        // TODO: end event data, e.g. total time spent, maybe whether it completed succesfully or
        // not
        self.emit_event_raw((), "_END".to_string()).await
    }

    pub(crate) async fn emit_error_event(
        &mut self,
        error: &GameError<C::Error>,
    ) -> Result<(), GameContextError> {
        // TODO: end event data, e.g. total time spent, maybe whether it completed succesfully or
        // not
        self.emit_event_raw(
            ErrorEvent {
                error: format!("{}", error),
                debug: format!("{:?}", error),
            },
            "_ERROR".to_string(),
        )
        .await
    }

    pub async fn forfeit_agent(&mut self, agent_id: usize) -> Result<(), GameContextError> {
        self.emit_event_raw(ForfeitEvent { agent_id }, "_FORFEIT".to_string())
            .await
    }

    /// Returns the number of agents playing in the game.
    /// Agents have IDs from 0 up to but not including `self.agents()`.
    pub fn agents(&self) -> usize {
        self.agents.len()
    }

    fn agent_mut(&mut self, agent_id: usize) -> Result<&mut VMAgent, GameContextError> {
        if agent_id >= self.agents() {
            return Err(GameContextError::UnknownAgent {
                id: agent_id,
                max: self.agents() - 1,
            });
        }

        Ok(&mut self.agents[agent_id])
    }

    /// Sets the timeout duration for waiting for the agent's next message.
    /// If `None` is passed in this becomes the DOXA default (`DEFAULT_MAX_MESSAGE_TIME`).
    pub fn set_max_message_time(&mut self, time: Option<Duration>) {
        self.max_message_time = time.unwrap_or(DEFAULT_MAX_MESSAGE_TIME);
    }

    /// Gets the next message from a particular agent.
    /// This method is cancel safe.
    ///
    /// This will timeout if it does not receive a message within `max_message_time` which can be
    /// configured.
    pub async fn next_message(&mut self, agent_id: usize) -> Result<&[u8], GameContextError> {
        let agent = self.agent_mut(agent_id)?;

        let msg = timeout(DEFAULT_MAX_MESSAGE_TIME, agent.next_message())
            .await
            .map_err(|_| GameContextError::TimeoutWaitingForMessage { agent_id })??;

        Ok(msg)
    }

    /// Takes a file from a particular directory, assuming that it exists.
    /// This does not wait for the file to be created or done writing, it's important that you
    /// allow the indicate it has written **and flushed** the file.
    /// This will delete the file it has taken.
    ///
    /// The recommended output directory is `/output` which is writable by the agent.
    pub async fn take_file<S: Into<String>>(
        &mut self,
        agent_id: usize,
        path: S,
    ) -> Result<Vec<u8>, GameContextError> {
        let agent = self.agent_mut(agent_id)?;

        let file = agent.take_file(path.into()).await?;

        Ok(file)
    }

    /// Sends a reboot message to the VM instructing it to restart the agent's process inside the
    /// VM.
    ///
    /// If the agent is not currently running then this just spawns the agent.
    /// It is possible to specify arguments for the agent.
    pub async fn reboot_agent(
        &mut self,
        agent_id: usize,
        args: Vec<String>,
    ) -> Result<(), GameContextError> {
        let agent = self.agent_mut(agent_id)?;

        agent.reboot(args).await?;

        Ok(())
    }

    /// Instructs every agent to reboot and waits until all of them have.
    pub async fn reboot_all_agents(&mut self, args: Vec<String>) -> Result<(), GameContextError> {
        for i in 0..self.agents() {
            self.reboot_agent(i, args.clone()).await?;
        }

        Ok(())
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
            .map_err(GameContextError::SendInput)?;

        Ok(())
    }

    /// Sends a message to all agents part of this competition.
    pub async fn broadcast_message_to_agents(
        &mut self,
        msg: &[u8],
    ) -> Result<(), GameContextError> {
        for i in 0..self.agents() {
            self.send_message_to_agent(i, msg).await?;
        }

        Ok(())
    }

    /// Expects there to be exactly `n` agents.
    /// If so it will return `Ok(())` otherwise it will return an error.
    pub fn expect_n_agents(&self, n: usize) -> Result<(), GameContextError> {
        if self.agents() == n {
            Ok(())
        } else {
            Err(GameContextError::IncorrectNumberAgents {
                expected: n,
                actual: self.agents(),
            })
        }
    }
}
