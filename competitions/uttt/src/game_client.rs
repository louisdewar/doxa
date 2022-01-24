use std::time::Duration;

use doxa_competition::{
    client::{async_trait, ForfeitError, GameClient, GameContext, GameError},
    tracing::debug,
};

use crate::model::{self, Model, ModelError, Player, Winner};

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct UTTTGameClient;

/// Games per side
/// In a pairing between A and B there are two matches one where A goes first and one where B goes first,
/// this number is the number of games played where A goes first and also the number of games where B goes first
const GAMES_PER_SIDE: u32 = 20;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Events that occur within a game
pub enum UTTTGameEvent {
    TilePlaced {
        #[serde(rename = "g")]
        grid: usize,
        #[serde(rename = "t")]
        tile: usize,
    },
    SmallGridWon {
        #[serde(rename = "g")]
        grid: usize,
        #[serde(rename = "w")]
        winner: Winner,
    },
    GameOver {
        #[serde(rename = "overall")]
        overall_winner: Winner,
    },
}

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UTTTMatchEvent {
    GameHistory {
        events: Vec<UTTTGameEvent>,
        overall_winner: Winner,
        #[serde(skip_serializing_if = "is_false")]
        #[serde(default = "bool::default")]
        forfeit: bool,
    },
    Scores {
        draws: u32,
        a_wins: u32,
        b_wins: u32,
    },
    GameWinners {
        winners: Vec<Winner>,
    },
}

#[derive(From, Error, Display, Debug, Clone)]
pub enum UTTTError {
    #[from]
    #[display(
        fmt = "model error from agent `{}`: {} (the agent output was: {})",
        agent,
        source,
        output
    )]
    Model {
        source: model::ModelError,
        agent: usize,
        output: String,
    },
    #[display(fmt = "improper format: agent {} outputted `{}`", agent, output)]
    ImproperFormat { agent: usize, output: String },
    #[display(
        fmt = "not a number: agent {} outputted `{}` where we expected a number",
        agent,
        output
    )]
    NotNumber { agent: usize, output: String },
}

impl ModelError {
    fn with_context(self, agent: usize, output: impl Into<String>) -> UTTTError {
        UTTTError::Model {
            source: self,
            agent,
            output: output.into(),
        }
    }
}

impl UTTTError {
    fn game_error(self) -> GameError<UTTTError> {
        GameError::Client(self)
    }
}

impl ForfeitError for UTTTError {
    fn forfeit(&self) -> Option<usize> {
        match &self {
            UTTTError::Model {
                source: e, agent, ..
            } => match e {
                ModelError::GameAlreadyOver => None,
                ModelError::WrongPlayer => None,
                ModelError::CellTaken => Some(*agent),
                ModelError::GridTaken => Some(*agent),
                ModelError::InvalidGrid => Some(*agent),
                ModelError::InvalidIndex => Some(*agent),
            },
            UTTTError::ImproperFormat { agent, .. } => Some(*agent),
            UTTTError::NotNumber { agent, .. } => Some(*agent),
        }
    }

    fn forfeit_message(&self) -> Option<String> {
        match &self {
            UTTTError::Model {
                source: e,
                output,
                ..
            } => match e {
                ModelError::GameAlreadyOver => None,
                ModelError::WrongPlayer => None,
                ModelError::CellTaken => Some(format!("The agent tried to play in a cell that was already taken (the agent outputted: {})", output)),
                ModelError::GridTaken => Some(format!("The agent tried to play in a grid that was already taken (the agent outputted: {})", output)),
                ModelError::InvalidGrid => Some(format!("The agent tried to play in an invalid grid (the agent outputted: {})", output)),
                ModelError::InvalidIndex => Some(format!("The agent tried to play in a grid or cell with an invalid (out of range) index (the agent outputted: {})", output)),
            },
            UTTTError::ImproperFormat { output, .. } => Some(format!(
                "We receieved `{}` from the agent which was not a properly formatted message",
                output
            )),
            UTTTError::NotNumber { output, .. } => Some(format!(
                "We recieved `{}` from the agent where we expected a number",
                output
            )),
        }
    }
}

impl UTTTGameClient {
    async fn run_once<E: FnMut(UTTTGameEvent)>(
        context: &mut GameContext<'_, Self>,
        mut on_event: E,
    ) -> Result<Winner, GameError<UTTTError>> {
        let mut model = Model::new();

        // 20 seconds to return a move
        context.set_max_message_time(Some(Duration::from_secs(20)));

        context.send_message_to_agent(0, b"S R\n").await?;
        context.send_message_to_agent(1, b"S B\n").await?;

        let mut current_player = Player::Red;

        loop {
            let (next_player, agent_id) = match current_player {
                Player::Red => (Player::Blue, 0),
                Player::Blue => (Player::Red, 1),
            };

            // Request action
            let playable_grids: String = model
                .playable_grids()
                .into_iter()
                .map(|grid| format!("{}", grid))
                .collect::<Vec<String>>()
                .join(",");

            context
                .send_message_to_agent(agent_id, format!("R {}\n", playable_grids).as_bytes())
                .await?;

            // Wait for their action
            let msg = context.next_message(agent_id).await?;

            // Parse their message
            let (grid, tile) = if let [start, grid, tile] =
                msg.split(|b| *b == b' ').collect::<Vec<_>>().as_slice()
            {
                if *start != b"M" {
                    return Err(UTTTError::ImproperFormat {
                        agent: agent_id,
                        output: String::from_utf8_lossy(msg).into(),
                    }
                    .game_error());
                }

                let grid = String::from_utf8_lossy(grid);
                let grid: usize = grid.parse().map_err(|_| {
                    UTTTError::NotNumber {
                        agent: agent_id,
                        output: grid.into(),
                    }
                    .game_error()
                })?;
                let tile = String::from_utf8_lossy(tile);
                let tile: usize = tile.parse().map_err(|_| {
                    UTTTError::NotNumber {
                        agent: agent_id,
                        output: tile.into(),
                    }
                    .game_error()
                })?;

                (grid, tile)
            } else {
                return Err(UTTTError::ImproperFormat {
                    agent: agent_id,
                    output: String::from_utf8_lossy(msg).into(),
                }
                .game_error());
            };

            let event = model.place_tile(current_player, grid, tile).map_err(|e| {
                e.with_context(agent_id, String::from_utf8_lossy(msg))
                    .game_error()
            })?;

            let place_msg = format!("P {} {} {}\n", current_player.to_char(), grid, tile);

            context
                .broadcast_message_to_agents(place_msg.as_bytes())
                .await?;

            on_event(UTTTGameEvent::TilePlaced { grid, tile });

            if let Some(event) = event {
                match event {
                    model::Event::GameOver {
                        overall_winner,
                        small_grid,
                        small_winner,
                    } => {
                        on_event(UTTTGameEvent::SmallGridWon {
                            grid: small_grid,
                            winner: small_winner,
                        });
                        on_event(UTTTGameEvent::GameOver { overall_winner });

                        return Ok(overall_winner);
                    }
                    model::Event::SmallGridWon { grid, winner } => {
                        context
                            .broadcast_message_to_agents(
                                format!("G {} {}\n", winner.to_char(), grid).as_bytes(),
                            )
                            .await?;
                        on_event(UTTTGameEvent::SmallGridWon { grid, winner });
                    }
                }
            }

            current_player = next_player;
        }
    }
}

#[async_trait]
impl GameClient for UTTTGameClient {
    type Error = UTTTError;

    type MatchRequest = ();

    type GameEvent = UTTTMatchEvent;

    async fn run<'a>(
        &self,
        _match_request: (),
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>> {
        context.expect_n_agents(2)?;

        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut draws = 0;
        let mut winners = Vec::with_capacity(GAMES_PER_SIDE as usize);
        debug!(total_games=%GAMES_PER_SIDE, "uttt starting");

        for game in 0..GAMES_PER_SIDE {
            // Game ID are 1 indexed as they are shown to the user
            let game_id = format!("game_{}", game + 1);
            // Reboot all agents to reset each game
            context.reboot_all_agents(vec![]).await?;

            let mut events = Vec::new();

            let overall_winner = match Self::run_once(context, |event| {
                events.push(event);
            })
            .await
            {
                Ok(winner) => winner,
                Err(e) => {
                    if let Some(agent) = e.forfeit() {
                        let remaining = GAMES_PER_SIDE - game;

                        let winner = if agent == 0 {
                            b_wins += remaining;
                            Winner::Blue
                        } else {
                            a_wins += remaining;
                            Winner::Red
                        };

                        context
                            .emit_game_event(
                                UTTTMatchEvent::GameHistory {
                                    overall_winner: winner,
                                    events,
                                    forfeit: true,
                                },
                                game_id,
                            )
                            .await?;

                        context
                            .emit_game_event(
                                UTTTMatchEvent::Scores {
                                    a_wins,
                                    b_wins,
                                    draws,
                                },
                                "scores",
                            )
                            .await?;

                        context
                            .emit_game_event(
                                UTTTMatchEvent::GameWinners { winners },
                                "game_winners",
                            )
                            .await?;
                    }

                    return Err(e);
                }
            };

            debug!(game_number=%game, "uttt game completed");

            context
                .emit_game_event(
                    UTTTMatchEvent::GameHistory {
                        overall_winner,
                        events,
                        forfeit: false,
                    },
                    game_id,
                )
                .await?;

            winners.push(overall_winner);
            match overall_winner {
                Winner::Red => a_wins += 1,
                Winner::Blue => b_wins += 1,
                Winner::Stalemate => draws += 1,
            }
        }

        context
            .emit_game_event(
                UTTTMatchEvent::Scores {
                    a_wins,
                    b_wins,
                    draws,
                },
                "scores",
            )
            .await?;

        context
            .emit_game_event(UTTTMatchEvent::GameWinners { winners }, "game_winners")
            .await?;

        Ok(())
    }
}
