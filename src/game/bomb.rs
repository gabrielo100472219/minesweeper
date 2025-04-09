use super::{Game, Position};
use super::helpers::get_adjacent_positions;
use super::helpers::get_all_positions;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::rng;

pub fn generate_bombs(game: &mut Game, start_position: Position, num_bombs: usize) {
    let forbidden_positions = get_adjacent_positions(start_position, game.width, game.height);
    let all_positions = get_all_positions(game.width, game.height);
    let forbidden_set: HashSet<Position> = forbidden_positions.iter().cloned().collect();
    let mut valid_positions: Vec<Position> = all_positions
        .into_iter()
        .filter(|pos| !forbidden_set.contains(pos))
        .collect();
    let mut rng = rng();
    valid_positions.shuffle(&mut rng);
    for i in 0..num_bombs {
        let x = valid_positions[i].x as usize;
        let y = valid_positions[i].y as usize;
        game.board[y][x].is_bomb = true;
    }
    game.bombs_placed = true;
}
