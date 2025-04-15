use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Direction, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Span, Line},
    Terminal,
};

use std::io;
use crate::game::{Cell, Game};

pub fn render_game(game: &Game, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let current_x = game.player_position.x as usize;
    let current_y = game.player_position.y as usize;
    terminal.draw(|f| {
        let size = f.size();
        let board_width = (game.width * 4) as u16;
        let board_height = game.height as u16 + 4;
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min((size.height.saturating_sub(board_height)) / 2),
                Constraint::Min(board_height),
                Constraint::Min(0),
            ])
            .split(size);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min((size.width.saturating_sub(board_width)) / 2),
                Constraint::Length(board_width),
                Constraint::Min(0),
            ])
            .split(vertical[1]);

        let area = horizontal[1];

        let mut lines: Vec<Line> = Vec::new();
        
        lines.push(Line::raw(""));

        for (y, row) in game.board.iter().enumerate() {
            let mut spans = vec![Span::raw("  ")];

            for (x, cell) in row.iter().enumerate() {
                let (symbol, color) = match (cell.is_open, cell.is_bomb, cell.is_flagged) {
                    (false, _, true) => ("[F]", Color::Red),
                    (false, _, false) => ("[?]", Color::White),
                    (true, true, _) => ("[*]", Color::Black),
                    (true, false, _) => {
                        if cell.adjacent_bombs == 0 {
                            ("[ ]", Color::Gray)
                        } else {
                            match cell.adjacent_bombs {
                                1 => ("[1]", Color::Blue),
                                2 => ("[2]", Color::Green),
                                3 => ("[3]", Color::Magenta),
                                4 => ("[4]", Color::Yellow),
                                5 => ("[5]", Color::Cyan),
                                6 => ("[6]", Color::White),
                                7 => ("[7]", Color::White),
                                8 => ("[8]", Color::White),
                                _ => ("[?]", Color::White),
                            }
                        }
                    }
                };


                let span = if x == current_x && y == current_y {
                    Span::styled(symbol, Style::default().fg(Color::Black).bg(Color::Yellow))
                } else {
                    Span::styled(symbol, Style::default().fg(color))
                };

                spans.push(span);
            }

            lines.push(Line::from(spans));
        }

        lines.push(Line::raw(""));

        let board = Paragraph::new(lines)
            .block(Block::default().title(" Minesweeper ").borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(board, area);
    })?;

    Ok(())
}

