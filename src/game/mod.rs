mod cell;
mod position;
mod bomb;
mod helpers;

use cell::Cell;
use position::Position;
use std::convert::TryInto;
use recursive::recursive;
use helpers::get_adjacent_positions;

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub number_of_mines: i32,
    pub board: Vec<Vec<Cell>>,
    pub player_position: Position,
    pub bombs_placed: bool,
    pub game_is_active: bool,
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
            player_position: initial_position,
            bombs_placed: false,
            game_is_active: true,
        }
    }

    pub fn generate_bombs(&mut self) {
        bomb::generate_bombs(self);
    }

    pub fn open_current_cell(&mut self) {
        if !self.game_is_active {
            return;
        }
        if !self.bombs_placed {
            self.generate_bombs();
        }
        let x: usize = self.player_position.x.try_into().expect("position x too large");
        let y: usize = self.player_position.y.try_into().expect("position y too large");
        if self.board[x][y].is_open {
            return;
        }
        if self.board[x][y].is_bomb {
            // Game ends
            self.game_is_active = false;
            return;
        }
        self.board[x][y].is_open = true;
        Self::open_adjacent_cells(&mut self.board, self.player_position, self.width, self.height);
    }

    #[recursive]
    fn open_adjacent_cells(board: &mut Vec<Vec<Cell>>, position: Position, width: usize, height: usize) {
        let adjacent_positions = get_adjacent_positions(position, width, height);
        for position in adjacent_positions.iter() {
            let adjacent_x: usize = position.x.try_into().expect("position x too large");
            let adjacent_y: usize = position.y.try_into().expect("position y too large");
            // This check should not be necessary, but just in case
            if board[adjacent_x][adjacent_y].is_bomb {
                continue;
            }
            // Cells with adjacent bombs are opened
            if board[adjacent_x][adjacent_y].adjacent_bombs != 0 {
                board[adjacent_x][adjacent_y].is_open = true;
                continue;
            }
            // If cell is empty, open it and its adjacents
            board[adjacent_x][adjacent_y].is_open = true;
            Self::open_adjacent_cells(board, *position, width, height);
        }
    }

    pub fn flag_current_cell(&mut self) {
        if !self.game_is_active {
            return;
        }
        let x: usize = self.player_position.x.try_into().expect("position x too large");
        let y: usize = self.player_position.y.try_into().expect("position y too large");
        if self.board[x][y].is_open {
            return;
        }
        self.board[x][y].is_flagged = !self.board[x][y].is_flagged;
    } 
}
