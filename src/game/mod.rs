mod cell;
mod position;
pub mod bomb;
pub mod helpers;

use cell::Cell;
use position::Position;
use std::convert::TryInto;

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub number_of_mines: i32,
    pub board: Vec<Vec<Cell>>,
    pub player_position: Position,
}

impl Game {
    pub fn new(width: usize, height: usize, number_of_mines: i32) -> Self {
        let board: Vec<Vec<Cell>> = (0..height)
            .map(|_| vec![Cell::new(); width])
            .collect();
        let width_i8: i8 = width.try_into().expect("width too large");
        let height_i8: i8 = height.try_into().expect("height too large");
        let initial_position: Position = Position::new(
            width_i8/2, 
            height_i8/2
        );
        Self { 
            width,
            height,
            number_of_mines,
            board,
            player_position: initial_position
        }
    }
}
