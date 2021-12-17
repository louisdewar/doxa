use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Owner {
    Empty,
    Red,
    Blue,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Player {
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "B")]
    Blue,
}

impl Player {
    pub fn to_char(self) -> char {
        match self {
            Player::Red => 'R',
            Player::Blue => 'B',
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Winner {
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "B")]
    Blue,
    #[serde(rename = "S")]
    Stalemate,
}

impl From<Player> for Owner {
    fn from(player: Player) -> Owner {
        match player {
            Player::Red => Owner::Red,
            Player::Blue => Owner::Blue,
        }
    }
}

impl Owner {
    pub fn to_winner(self) -> Option<Winner> {
        match self {
            Owner::Empty => None,
            Owner::Red => Some(Winner::Red),
            Owner::Blue => Some(Winner::Blue),
        }
    }
}

impl Winner {
    pub fn stalemate(self) -> bool {
        matches!(self, Winner::Stalemate)
    }

    pub fn to_char(self) -> char {
        match self {
            Winner::Red => 'R',
            Winner::Blue => 'B',
            Winner::Stalemate => 'S',
        }
    }
}

#[derive(Clone, Copy)]
pub struct SmallGrid {
    inner: [Owner; 9],
    winner: Option<Winner>,
}

impl SmallGrid {
    fn new() -> Self {
        SmallGrid {
            inner: [Owner::Empty; 9],
            winner: None,
        }
    }

    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    pub fn find_winner(&mut self) -> Option<Winner> {
        if let Some(winner) = find_winner(&self.inner) {
            self.winner = winner.into();
            Some(winner)
        } else {
            None
        }
    }
}

impl Index<usize> for SmallGrid {
    type Output = Owner;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for SmallGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

pub struct Grid {
    /// Data layout:
    /// [[SMALL GRID 0 (9)], [SMALL GRID 1 (9)], ..., [SMALL GRID 8 (9)]]
    inner: [SmallGrid; 9],
    winner: Option<Winner>,
}

impl Index<usize> for Grid {
    type Output = SmallGrid;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            inner: [SmallGrid::new(); 9],
            winner: None,
        }
    }

    pub fn find_winner(&mut self) -> Option<Winner> {
        if let Some(winner) = find_winner(&self.inner) {
            self.winner = Some(winner);
            Some(winner)
        } else {
            None
        }
    }
}

pub trait Winnable {
    fn winner(&self) -> Option<Winner>;
}

impl Winnable for Owner {
    fn winner(&self) -> Option<Winner> {
        self.to_winner()
    }
}

impl Winnable for SmallGrid {
    fn winner(&self) -> Option<Winner> {
        self.winner
    }
}

impl Winnable for Grid {
    fn winner(&self) -> Option<Winner> {
        self.winner
    }
}

fn find_winner<W: Winnable>(grid: &[W; 9]) -> Option<Winner> {
    // For the purposes of this function assume that the data is layed out in rows ([[row], [row],
    // [row]]. It doesn't really matter as it's symmetrical.
    // Therefore f(x, y) -> index: x + y * 3

    let w = |a, b, c| {
        {
            // Determine if there is a stalemate:
            // - if any of the three are a Stalemate then it is impossible to win this set
            // - if one tile is won by one player and another tile by another player then there is
            // also a stalemate (no one can win by these three)
            let mut found_player = None;
            for i in [a, b, c] {
                let current_player = match Winnable::winner(&grid[i]) {
                    Some(Winner::Stalemate) => return Some(Winner::Stalemate),
                    Some(Winner::Red) => Player::Red,
                    Some(Winner::Blue) => Player::Blue,
                    None => continue,
                };

                if let Some(found_player) = found_player {
                    // There are two different players on this tile
                    if found_player != current_player {
                        return Some(Winner::Stalemate);
                    }
                } else {
                    found_player = Some(current_player);
                }
            }

            // All three were empty
            found_player?;
        }

        let a = Winnable::winner(&grid[a])?;
        let b = Winnable::winner(&grid[b])?;
        let c = Winnable::winner(&grid[c])?;

        if a == b && b == c {
            Some(a)
        } else {
            None
        }
    };

    let pos = |x: usize, y: usize| {
        assert!(x < 3);
        assert!(y < 3);

        x + y * 3
    };

    // If there is at least one trio where it is possible for a player to win then set this to
    // false
    let mut stalemate = true;

    // Check horizontals
    for y in 0..3 {
        if let Some(winner) = w(pos(0, y), pos(1, y), pos(2, y)) {
            if winner.stalemate() {
                continue;
            }
            return Some(winner);
        } else {
            stalemate = false;
        }
    }

    // Check verticals
    for x in 0..3 {
        if let Some(winner) = w(pos(x, 0), pos(x, 1), pos(x, 2)) {
            if winner.stalemate() {
                continue;
            }
            return Some(winner);
        } else {
            stalemate = false;
        }
    }

    // Check diagonals
    for [a, b, c] in [[0, 1, 2], [2, 1, 0]] {
        // Checks [(0,0), (1,1), (2,2)] then [(2,0), (1,1), (0,2)]
        if let Some(winner) = w(pos(a, 0), pos(b, 1), pos(c, 2)) {
            if winner.stalemate() {
                continue;
            }
            return Some(winner);
        } else {
            stalemate = false;
        }
    }

    if stalemate {
        return Some(Winner::Stalemate);
    }

    None
}
