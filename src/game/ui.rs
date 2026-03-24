use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};

use crate::game::{Game, GameState};
use std::io;

pub fn render_game(
    game: &Game,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    let current_x = game.player_position.x as usize;
    let current_y = game.player_position.y as usize;
    terminal.draw(|f| {
        let size = f.area();
        let (area, _) = board_layout(size, game);

        let lines = build_board_lines(game, current_x, current_y);
        let board = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Minesweeper ")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center);

        f.render_widget(board, area);
    })?;

    Ok(())
}

pub fn render_end_screen(
    game: &Game,
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    let current_x = game.player_position.x as usize;
    let current_y = game.player_position.y as usize;
    terminal.draw(|f| {
        let size = f.area();
        let (area, _) = board_layout(size, game);

        // Draw the board in the background (revealed state)
        let lines = build_board_lines(game, current_x, current_y);
        let board = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" Minesweeper ")
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center);
        f.render_widget(board, area);

        // Draw the overlay popup centered on the board
        let popup_width: u16 = 38;
        let popup_height: u16 = 9;
        let popup_area = centered_rect(popup_width, popup_height, area);

        let (title, message_lines, border_color) = match game.state {
            GameState::Lost => (
                " GAME OVER ",
                vec![
                    Line::raw(""),
                    Line::styled(
                        "BOOM! You hit a mine!",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Line::raw(""),
                    Line::styled("Better luck next time.", Style::default().fg(Color::Gray)),
                    Line::raw(""),
                    Line::styled(
                        "'r' to restart | 'q' to exit",
                        Style::default().fg(Color::DarkGray),
                    ),
                    Line::raw(""),
                ],
                Color::Red,
            ),
            GameState::Won => (
                " YOU WIN! ",
                vec![
                    Line::raw(""),
                    Line::styled(
                        "Congratulations!",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Line::raw(""),
                    Line::styled(
                        "You cleared the minefield!",
                        Style::default().fg(Color::Yellow),
                    ),
                    Line::raw(""),
                    Line::styled(
                        "'r' to restart | 'q' to exit",
                        Style::default().fg(Color::DarkGray),
                    ),
                    Line::raw(""),
                ],
                Color::Green,
            ),
            _ => unreachable!(),
        };

        // Clear the popup area so the board doesn't bleed through
        f.render_widget(Clear, popup_area);

        let popup = Paragraph::new(message_lines)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(border_color)),
            )
            .alignment(Alignment::Center);
        f.render_widget(popup, popup_area);
    })?;

    Ok(())
}

/// Computes the centered board layout area. Returns (area, board_width).
fn board_layout(size: Rect, game: &Game) -> (Rect, u16) {
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

    (horizontal[1], board_width)
}

fn build_board_lines(game: &Game, current_x: usize, current_y: usize) -> Vec<Line<'static>> {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::raw(""));

    for (y, row) in game.board.iter().enumerate() {
        let mut spans = vec![Span::raw("  ")];

        for (x, cell) in row.iter().enumerate() {
            let (symbol, color) = match (cell.is_open, cell.is_bomb, cell.is_flagged) {
                (false, _, true) => ("[F]", Color::Red),
                (false, _, false) => ("[?]", Color::White),
                (true, true, _) => ("[*]", Color::Red),
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
    lines
}

/// Returns a centered `Rect` of the given dimensions within `area`.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
