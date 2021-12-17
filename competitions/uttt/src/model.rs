mod grid;

use grid::{Grid, Winnable};

use derive_more::{Display, Error};

pub use grid::{Player, Winner};

#[cfg(test)]
mod test;

#[derive(Debug, Display, Error, Clone)]
pub enum ModelError {
    #[display(fmt = "the game was already over before this turn")]
    GameAlreadyOver,
    #[display(fmt = "the wrong player tried to play their turn")]
    WrongPlayer,
    #[display(fmt = "tried to place their tile where there already was one")]
    CellTaken,
    #[display(fmt = "tried to place their tile in a grid that already had a winner")]
    GridTaken,
    #[display(fmt = "tried to place a tile in a grid when it was not allowed")]
    InvalidGrid,
    #[display(fmt = "index for grid/cell is out of range")]
    InvalidIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    SmallGridWon {
        grid: usize,
        winner: Winner,
    },
    /// Whenever there is a game over this is also precipitated by a small grid being won.
    /// Because of stalemates it is not technically the case that the `small_winner ==
    /// overall_winner` if overall_winner is Stalemate, but in all other cases they will equal.
    GameOver {
        overall_winner: Winner,
        small_grid: usize,
        small_winner: Winner,
    },
}

pub struct Model {
    grid: Grid,
    next_player: Player,
    next_grid: Option<usize>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            grid: Grid::new(),
            next_player: Player::Red,
            next_grid: None,
        }
    }

    pub fn place_tile(
        &mut self,
        player: Player,
        grid: usize,
        cell: usize,
    ) -> Result<Option<Event>, ModelError> {
        if self.grid.winner().is_some() {
            return Err(ModelError::GameAlreadyOver);
        }

        if player != self.next_player {
            return Err(ModelError::WrongPlayer);
        }

        self.next_player = match player {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        };

        if !(grid < 9 && cell < 9) {
            return Err(ModelError::InvalidIndex);
        }

        if let Some(next_grid) = self.next_grid {
            if grid != next_grid {
                return Err(ModelError::InvalidGrid);
            }
        }

        let small = &mut self.grid[grid];

        if small.winner().is_some() {
            return Err(ModelError::GridTaken);
        }

        if small[cell].winner().is_some() {
            return Err(ModelError::CellTaken);
        }

        small[cell] = player.into();

        if self.grid[cell].winner().is_some() {
            self.next_grid = None;
        } else {
            self.next_grid = Some(cell);
        }

        if let Some(small_winner) = self.grid[grid].find_winner() {
            // We've just won this grid so it can't be where the next player plays:
            if grid == cell {
                self.next_grid = None;
            }

            if let Some(overall_winner) = self.grid.find_winner() {
                return Ok(Some(Event::GameOver {
                    overall_winner,
                    small_grid: grid,
                    small_winner,
                }));
            }

            return Ok(Some(Event::SmallGridWon {
                grid,
                winner: small_winner,
            }));
        }

        Ok(None)
    }

    pub fn playable_grids(&self) -> Vec<usize> {
        if self.grid.winner().is_some() {
            return Vec::new();
        }

        if let Some(next_grid) = self.next_grid {
            assert_eq!(self.grid[next_grid].winner(), None);
            vec![next_grid]
        } else {
            (0..9)
                .filter(|i| self.grid[*i].winner().is_none())
                .collect()
        }
    }
}
