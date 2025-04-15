mod cell;
mod position;
mod bomb;
mod helpers;
pub mod ui;
pub mod input;

use cell::Cell;
use position::Position;
use std::convert::TryInto;
use recursive::recursive;
use helpers::get_adjacent_positions;

pub struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<Cell>>,
    pub player_position: Position,
    bombs_placed: bool,
    pub game_is_active: bool,
    number_of_open_cells: i64,
    total_number_of_cells: i64,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let board: Vec<Vec<Cell>> = (0..height)
            .map(|_| vec![Cell::new(); width])
            .collect();
        let width_i8: i8 = width.try_into().expect("width too large");
        let height_i8: i8 = height.try_into().expect("height too large");
        let initial_position: Position = Position::new(
            width_i8/2 - 1,
            height_i8/2 -1
        );
        let number_of_open_cells = 0;
        let total_number_of_cells = width as i64 * height as i64;
        Self { 
            width,
            height,
            board,
            player_position: initial_position,
            bombs_placed: false,
            game_is_active: true,
            total_number_of_cells,
            number_of_open_cells,
        }
    }

    fn generate_bombs(&mut self) {
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
        if self.board[y][x].is_open {
            return;
        }
        if self.board[y][x].is_bomb {
            // Game ends
            self.game_is_active = false;
            return;
        }
        self.board[y][x].is_open = true;
        self.number_of_open_cells += 1;
        if (self.board[y][x].adjacent_bombs > 0) {
            return;
        }
        let opened_cells = Self::open_adjacent_cells(&mut self.board, self.player_position, self.width, self.height);
        self.number_of_open_cells += opened_cells;
    }

    #[recursive]
    fn open_adjacent_cells(board: &mut Vec<Vec<Cell>>, position: Position, width: usize, height: usize) -> i64 {
        let adjacent_positions = get_adjacent_positions(position, width, height);
        let mut opened_cells = 0;
        for position in adjacent_positions.iter() {
            let adjacent_x: usize = position.x.try_into().expect("position x too large");
            let adjacent_y: usize = position.y.try_into().expect("position y too large");
            if board[adjacent_y][adjacent_x].is_open {
                continue;
            }
            // This check should not be necessary, but just in case
            if board[adjacent_y][adjacent_x].is_bomb {
                continue;
            }
            // Cells with adjacent bombs are opened
            if board[adjacent_y][adjacent_x].adjacent_bombs != 0 {
                board[adjacent_y][adjacent_x].is_open = true;
                opened_cells += 1;
                continue;
            }
            // If cell is empty, open it and its adjacents
            board[adjacent_y][adjacent_x].is_open = true;
            opened_cells += 1;
            let opened_adjacent_cells = Self::open_adjacent_cells(board, *position, width, height);
            opened_cells += opened_adjacent_cells;
        }
        opened_cells
    }

    pub fn flag_current_cell(&mut self) {
        if !self.game_is_active {
            return;
        }
        let x: usize = self.player_position.x.try_into().expect("position x too large");
        let y: usize = self.player_position.y.try_into().expect("position y too large");
        if self.board[y][x].is_open {
            return;
        }
        self.board[y][x].is_flagged = !self.board[y][x].is_flagged;
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use cell::Cell;
    use position::Position;

    // Helper to count how many cells are open
    fn count_open_cells(board: &Vec<Vec<Cell>>) -> usize {
        board.iter()
            .map(|row| row.iter().filter(|c| c.is_open).count())
            .sum()
    }

    #[test]
    fn board_created_with_correct_stats() {
        let game = Game::new(10, 10);
        let middle_pos = Position::new(4, 4); // (width/2 -1, height/2 -1)
        assert_eq!(game.player_position, middle_pos);
        assert!(game.game_is_active);
        assert_eq!(game.total_number_of_cells, 100);
        assert_eq!(game.number_of_open_cells, 0);
    }

    #[test]
    fn flag_cell_toggles_correctly() {
        let mut game = Game::new(5, 5);
        let x = game.player_position.x as usize;
        let y = game.player_position.y as usize;

        // Initially not flagged
        assert!(!game.board[x][y].is_flagged);
        game.flag_current_cell();
        assert!(game.board[x][y].is_flagged);
        game.flag_current_cell();
        assert!(!game.board[x][y].is_flagged);
    }

    #[test]
    fn open_first_cell_opens_some_cells() {
        let mut game = Game::new(10, 10);
        game.open_current_cell();

        // First open will generate bombs and open the selected cell
        assert!(game.board[game.player_position.x as usize][game.player_position.y as usize].is_open);
        assert!(game.bombs_placed);

        // The number of open cells should be greater than zero
        assert!(game.number_of_open_cells > 0);

        // Total open cells on the board match game.number_of_open_cells
        let count = count_open_cells(&game.board);
        assert_eq!(count as i64, game.number_of_open_cells);
    }

    #[test]
    fn open_cell_on_bomb_ends_game() {
        let mut game = Game::new(5, 5);
        let x = game.player_position.x as usize;
        let y = game.player_position.y as usize;

        // Manually set a bomb in the starting position
        game.board[x][y].is_bomb = true;
        game.bombs_placed = true;

        game.open_current_cell();
        assert!(!game.game_is_active); // Game should end
    }

    #[test]
    fn open_already_open_cell_does_nothing() {
        let mut game = Game::new(3, 3);
        // Open the cell
        game.open_current_cell();
        let open_count_before = game.number_of_open_cells;

        // Try opening again
        game.open_current_cell();
        let open_count_after = game.number_of_open_cells;

        assert_eq!(open_count_before, open_count_after);
    }

    #[test]
    fn flag_does_not_change_open_cells() {
        let mut game = Game::new(3, 3);
        game.flag_current_cell();
        assert_eq!(game.number_of_open_cells, 0);
    }

    #[test]
    fn flag_does_nothing_on_open_cell() {
        let mut game = Game::new(3, 3);
        game.open_current_cell();
        let x = game.player_position.x as usize;
        let y = game.player_position.y as usize;
        assert!(game.board[x][y].is_open);

        game.flag_current_cell();
        assert!(!game.board[x][y].is_flagged); // Still not flagged
    }
}

