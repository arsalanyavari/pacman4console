use std::io::{Write};
use crossterm::{
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use crate::game_state::{GameState, Tile, ROWS, COLS};

pub fn draw_world<W: Write>(state: &GameState, out: &mut W) -> std::io::Result<()> {
    // Clear the screen
    queue!(out, Clear(ClearType::All))?;

    // Draw the board
    for r in 0..ROWS {
        for c in 0..COLS {
            queue!(out, MoveTo(c as u16, r as u16))?;
            let tile = state.level[r][c];
            match tile {
                Tile::Wall => queue!(out, Print("#"))?,
                Tile::Pellet => queue!(out, Print("."))?,
                Tile::PowerUp => queue!(out, Print("*"))?,
                Tile::GhostWall => queue!(out, Print("="))?,
                _ => queue!(out, Print(" "))?,
            }
        }
    }

    // Draw ghosts
    let ghost_colors = [Color::Red, Color::Yellow, Color::Magenta, Color::Blue];
    for (i, g) in state.ghosts.iter().enumerate() {
        queue!(out, MoveTo(g.col as u16, g.row as u16))?;
        let color = ghost_colors.get(i).copied().unwrap_or(Color::White);
        queue!(out, SetForegroundColor(color))?;

        if state.powerd_up {
            if let Some(end) = state.power_end_time {
                let secs_left = end.saturating_duration_since(std::time::Instant::now()).as_secs();
                let digit = secs_left.min(9); // only 1 digit
                queue!(out, Print(digit.to_string()))?;
            } else {
                queue!(out, Print("8"))?;
            }
        } else {
            queue!(out, Print("&"))?;
        }
        queue!(out, ResetColor)?;
    }

    // Draw Pac-Man
    queue!(
        out,
        MoveTo(state.pac_col as u16, state.pac_row as u16),
        SetForegroundColor(Color::Yellow),
        Print("C"),
        ResetColor
    )?;

    draw_status(state, out)?;

    out.flush()?;
    Ok(())
}

/// Draws status bar
fn draw_status<W: Write>(state: &GameState, out: &mut W) -> std::io::Result<()> {
    let status_row = ROWS as u16 + 1;
    queue!(out, MoveTo(0, status_row))?;
    let status_text = format!(
        "Level: {}  Score: {}  Lives: {}  Food: {}",
        state.level_number, state.score, state.lives, state.food_left
    );
    queue!(out, Print(status_text))?;
    Ok(())
}
