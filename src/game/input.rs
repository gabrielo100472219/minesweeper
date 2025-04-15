use crossterm::event::KeyCode;
use crate::game::{Game, Position};

pub fn handle_input(game: &mut Game, key: KeyCode) {
    let Position { x, y } = game.player_position;
    let (max_x, max_y) = (game.width as i8 - 1, game.height as i8 - 1);

    match key {
        KeyCode::Char('h') => {
            if x > 0 {
                game.player_position.x -= 1;
            }
        }
        KeyCode::Char('l') => {
            if x < max_x {
                game.player_position.x += 1;
            }
        }
        KeyCode::Char('k') => {
            if y > 0 {
                game.player_position.y -= 1;
            }
        }
        KeyCode::Char('j') => {
            if y < max_y {
                game.player_position.y += 1;
            }
        }
        KeyCode::Char('f') => {
            game.flag_current_cell();
        }
        KeyCode::Char('o') => {
            game.open_current_cell();
        }
        _ => {}
    }
}
