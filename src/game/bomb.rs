use super::{Game, Position};
use super::helpers::get_adjacent_positions;
use super::helpers::get_all_positions;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::rng;

pub fn generate_bombs(game: &mut Game) {
    let forbidden_positions = get_adjacent_positions(game.player_position, game.width, game.height);
    let all_positions = get_all_positions(game.width, game.height);
    let forbidden_set: HashSet<Position> = forbidden_positions.iter().cloned().collect();
    let mut valid_positions: Vec<Position> = all_positions
        .into_iter()
        .filter(|pos| !forbidden_set.contains(pos))
        .collect();
    let mut rng = rng();
    valid_positions.shuffle(&mut rng);
    let total_squares = game.height * game.width;
    let num_bombs = (total_squares as f64 * 0.15).round() as usize; 
    for i in 0..num_bombs {
        let x = valid_positions[i].x as usize;
        let y = valid_positions[i].y as usize;
        game.board[y][x].is_bomb = true;
        increase_adjacency_count_of_surrounding_cells(game, game.player_position);
    }
    game.bombs_placed = true;
}

fn increase_adjacency_count_of_surrounding_cells(game: &mut Game, position: Position) {
    let adjacent_positions = get_adjacent_positions(position, game.width, game.height);
    for position in adjacent_positions.iter(){
        let adjacent_x: usize = position.x.try_into().expect("position x too large");
        let adjacent_y: usize = position.y.try_into().expect("position y too large");
        game.board[adjacent_x][adjacent_y].adjacent_bombs += 1;
    }
} 
