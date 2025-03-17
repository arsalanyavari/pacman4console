use std::io::stdout;
use std::time::{Duration, Instant};

use crossterm::{
    cursor::Hide,
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::game_state::{GameState, SPEED_OF_GAME};
use crate::movement::{move_pacman, move_ghosts, check_collision};
use crate::draw::draw_world;


mod game_state;
mod movement;
mod draw;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <level_number>", args[0]);
        std::process::exit(1);
    }

    let level_num: u8 = match args[1].parse() {
        Ok(n) if (1..=9).contains(&n) => n,
        _ => {
            eprintln!("Please provide a level number between 1 and 9.");
            std::process::exit(1);
        }
    };

    // Set up terminal
    let mut stdout = stdout();
    execute!(stdout, Hide)?;
    enable_raw_mode()?;

    let mut state = GameState::new();

    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let level_filename = format!("level0{}.dat", level_num);
    let level_path = exe_dir.join("Levels").join(level_filename);

    if let Err(e) = state.load_level_from_file(level_path.to_string_lossy().as_ref()) {
        eprintln!("Cannot load level file ({}): {}", level_path.display(), e);
        eprintln!("Ensure you have a 'Levels' folder next to the executable.");
        disable_raw_mode()?;
        std::process::exit(1);
    }

    // game loop
    loop {
        if poll(Duration::from_millis(10))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('w') => {
                        state.pac_dir_r = -1;
                        state.pac_dir_c = 0;
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        state.pac_dir_r = 1;
                        state.pac_dir_c = 0;
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        state.pac_dir_r = 0;
                        state.pac_dir_c = -1;
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        state.pac_dir_r = 0;
                        state.pac_dir_c = 1;
                    }
                    KeyCode::Char('p') => {
                        loop {
                            if let Event::Key(k2) = read()? {
                                if k2.code == KeyCode::Char('p') {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        move_pacman(&mut state);
        check_collision(&mut state); // Huh BUG fix :D

        move_ghosts(&mut state);
        check_collision(&mut state);

        if state.lives < 0 {
            break;
        }

        if let Some(end) = state.power_end_time {
            if Instant::now() >= end {
                state.powerd_up = false;
                state.ghosts_in_a_row = 0;
                state.power_end_time = None;
            }
        }

        if state.score > state.free_life {
            state.lives += 1;
            state.free_life *= 2;
        }

        if state.food_left <= 0 {
            break;
        }

        draw_world(&state, &mut stdout)?;

        // wait
        std::thread::sleep(Duration::from_millis(SPEED_OF_GAME));
    }

    execute!(stdout, crossterm::cursor::Show)?;
    disable_raw_mode()?;

    println!("Game Over! Final Score: {}", state.score);
    Ok(())
}
