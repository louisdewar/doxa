use doxa_competition::client::{async_trait, ForfeitError, GameClient, GameContext, GameError};

use crate::model::{self, Model, Player, Winner};

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

pub struct UTTTGameClient;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UTTTGameEvent {
    TilePlaced {
        grid: usize,
        tile: usize,
        player: Player,
    },
    SmallGridWon {
        grid: usize,
        winner: Winner,
    },
    GameOver {
        winner: Winner,
    },
    Scores {
        draws: u32,
        a_wins: u32,
        b_wins: u32,
    },
}

// TODO: forfeit for agent with error

#[derive(From, Error, Display, Debug, Clone)]
pub enum UTTTError {
    #[from]
    Model(model::ModelError),
    ImproperFormat {
        agent: usize,
    },
    NotNumber {
        agent: usize,
    },
}

impl UTTTError {
    fn game_error(self) -> GameError<UTTTError> {
        GameError::Client(self)
    }
}

impl ForfeitError for UTTTError {
    fn forfeit(&self) -> Option<usize> {
        match &self {
            UTTTError::Model(_) => None,
            UTTTError::ImproperFormat { agent } => Some(*agent),
            UTTTError::NotNumber { agent } => Some(*agent),
        }
    }
}

impl UTTTGameClient {
    async fn run_once<'a, E: FnMut(UTTTGameEvent)>(
        context: &mut GameContext<'a, Self>,
        mut on_event: E,
    ) -> Result<(), GameError<UTTTError>> {
        let mut model = Model::new();

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
                    return Err(UTTTError::ImproperFormat { agent: agent_id }.game_error());
                }

                let grid: usize = String::from_utf8_lossy(grid)
                    .parse()
                    .map_err(|_| UTTTError::NotNumber { agent: agent_id }.game_error())?;
                let tile: usize = String::from_utf8_lossy(tile)
                    .parse()
                    .map_err(|_| UTTTError::NotNumber { agent: agent_id }.game_error())?;

                (grid, tile)
            } else {
                return Err(UTTTError::ImproperFormat { agent: agent_id }.game_error());
            };

            let event = model
                .place_tile(current_player, grid, tile)
                .map_err(|e| UTTTError::from(e).game_error())?;

            let place_msg = format!("P {} {} {}\n", current_player.to_char(), grid, tile);

            context
                .broadcast_message_to_agents(place_msg.as_bytes())
                .await?;

            on_event(UTTTGameEvent::TilePlaced {
                grid,
                tile,
                player: current_player,
            });
            // context
            //     .emit_game_event(
            //         UTTGameEvent::TilePlaced {
            //             grid,
            //             tile,
            //             player: current_player,
            //         },
            //         event_id.clone(),
            //     )
            //     .await?;

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
                        on_event(UTTTGameEvent::GameOver {
                            winner: overall_winner,
                        });
                        // context
                        //     .emit_game_event(
                        //         UTTGameEvent::SmallGridWon {
                        //             grid: small_grid,
                        //             winner: small_winner,
                        //         },
                        //         event_id.clone(),
                        //     )
                        //     .await?;
                        // context
                        //     .emit_game_event(
                        //         UTTGameEvent::GameOver {
                        //             winner: overall_winner,
                        //         },
                        //         event_id.clone(),
                        //     )
                        //     .await?;

                        return Ok(());
                    }
                    model::Event::SmallGridWon { grid, winner } => {
                        context
                            .broadcast_message_to_agents(
                                format!("G {}", winner.to_char()).as_bytes(),
                            )
                            .await?;
                        on_event(UTTTGameEvent::SmallGridWon { grid, winner });
                        // context
                        //     .emit_game_event(
                        //         UTTGameEvent::SmallGridWon { grid, winner },
                        //         event_id.clone(),
                        //     )
                        //     .await?;
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

    type GameEvent = UTTTGameEvent;

    async fn run<'a>(
        _match_request: (),
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>> {
        context.expect_n_agents(2)?;

        let mut a_wins = 0;
        let mut b_wins = 0;
        let mut draws = 0;

        let mut events = Vec::new();
        for game in 0..100 {
            events.truncate(0);
            // Game ID are 1 indexed as they are shown to the user
            let game_id = format!("game_{}", game + 1);

            if let Err(e) = Self::run_once(context, |event| {
                events.push(event);
            })
            .await
            {
                if let Some(agent) = e.forfeit() {
                    let remaining = 100 - game;
                    if agent == 0 {
                        b_wins += remaining;
                    } else {
                        a_wins += remaining;
                    }

                    context
                        .emit_game_event(
                            UTTTGameEvent::Scores {
                                a_wins,
                                b_wins,
                                draws,
                            },
                            "scores",
                        )
                        .await?;
                }

                return Err(e);
            }

            for event in &events {
                if let &UTTTGameEvent::GameOver { winner } = event {
                    match winner {
                        Winner::Red => a_wins += 1,
                        Winner::Blue => b_wins += 1,
                        Winner::Stalemate => draws += 1,
                    }
                }

                context
                    .emit_game_event(event.clone(), game_id.clone())
                    .await?;
            }
        }

        context
            .emit_game_event(
                UTTTGameEvent::Scores {
                    a_wins,
                    b_wins,
                    draws,
                },
                "scores",
            )
            .await?;

        // Self::run_once(context, |context, event| async move {
        //     context.emit_game_event(event, "game_1").await?;

        //     Ok(())
        // })
        // .await

        Ok(())
    }
}
