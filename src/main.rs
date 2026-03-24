use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Result};

mod game;
use game::GameState;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = game::Game::new(15, 15);

    'outer: loop {
        // Main game loop
        while game.state == GameState::Playing {
            game::ui::render_game(&game, &mut terminal)?;
            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            game.state = GameState::Quit;
                        }
                        _ => {
                            game::input::handle_input(&mut game, key.code);
                        }
                    }
                }
            }
        }

        // End screen loop — show win/lose message until user presses q or r
        if game.state == GameState::Won || game.state == GameState::Lost {
            loop {
                game::ui::render_end_screen(&game, &mut terminal)?;
                if event::poll(std::time::Duration::from_millis(200))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => break 'outer,
                            KeyCode::Char('r') => {
                                game = game::Game::new(15, 15);
                                break; // break inner loop, restart outer
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            // Quit state
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
