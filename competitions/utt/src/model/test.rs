use crate::model::Event;
use crate::model::Winner;

use super::Model;
use super::Player::*;

#[test]
fn test_win_grid_diag() {
    let mut model = Model::new();

    assert!(model.place_tile(Red, 0, 0).unwrap().is_none());
    assert!(model.place_tile(Blue, 0, 1).unwrap().is_none());
    assert!(model.place_tile(Red, 0, 4).unwrap().is_none());
    assert!(model.place_tile(Blue, 0, 2).unwrap().is_none());
    assert_eq!(
        model.place_tile(Red, 0, 8).unwrap().unwrap(),
        Event::SmallGridWon {
            grid: 0,
            winner: Winner::Red
        }
    );
}
