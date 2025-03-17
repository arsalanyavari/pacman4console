use std::time::{Duration, Instant};

use rand::{Rng};
use rand::rng;

use crate::game_state::{
    Tile, GameState, Ghost, ROWS, COLS, HOW_SLOW
};

pub fn move_pacman(state: &mut GameState) {
    let mut new_r = state.pac_row + state.pac_dir_r;
    let mut new_c = state.pac_col + state.pac_dir_c;

    // Wrap vertical
    if new_r < 0 {
        new_r = ROWS as i16 - 1;
    } else if new_r >= ROWS as i16 {
        new_r = 0;
    }

    if new_c < 0 {
        new_c = COLS as i16 - 1;
    } else if new_c >= COLS as i16 {
        new_c = 0;
    }

    let tile = state.level[new_r as usize][new_c as usize];
    if tile == Tile::Wall || tile == Tile::GhostWall {
        return;
    }

    state.pac_row = new_r;
    state.pac_col = new_c;

    match tile {
        Tile::Pellet => {
            state.score += 1;
            state.food_left -= 1;
            state.level[new_r as usize][new_c as usize] = Tile::Empty;
        }
        Tile::PowerUp => {
            state.score += 5;
            state.level[new_r as usize][new_c as usize] = Tile::Empty;
            state.powerd_up = true;
            if state.ghosts_in_a_row == 0 {
                state.ghosts_in_a_row = 1;
            }
            state.power_end_time = Some(Instant::now() + Duration::from_secs(5));
        }
        _ => {}
    }
}


pub fn move_ghosts(state: &mut GameState) {
    // Pac-Man is powered up slow down the ghosts
    if state.powerd_up {
        state.slower_ghosts_counter += 1;
        if state.slower_ghosts_counter <= HOW_SLOW {
            return;
        }
        state.slower_ghosts_counter = 0;
    }

    let level = &state.level;
    let mut rng = rng();
    let chase_prob = state.chase_probability;

    for g in &mut state.ghosts {
        let mut new_r = g.row + g.dir_r;
        let mut new_c = g.col + g.dir_c;

        if new_r < 0 {
            new_r = ROWS as i16 - 1;
        } else if new_r >= ROWS as i16 {
            new_r = 0;
        }
        if new_c < 0 {
            new_c = COLS as i16 - 1;
        } else if new_c >= COLS as i16 {
            new_c = 0;
        }

        let is_blocked = is_wall(level, new_r, new_c);
        if is_blocked {
            let possible_dirs = collect_possible_ghost_dirs(g.row, g.col, level);
            if !possible_dirs.is_empty() {
                if rng.random::<f32>() < chase_prob {
                    if let Some(chase_dir) = pick_best_chase_dir(
                        g.row,
                        g.col,
                        state.pac_row,
                        state.pac_col,
                        &possible_dirs,
                    ) {
                        g.dir_r = chase_dir.0;
                        g.dir_c = chase_dir.1;
                    } else {
                        let idx = rng.random_range(0..possible_dirs.len());
                        let (dr, dc) = possible_dirs[idx];
                        g.dir_r = dr;
                        g.dir_c = dc;
                    }
                } else {
                    let idx = rng.random_range(0..possible_dirs.len());
                    let (dr, dc) = possible_dirs[idx];
                    g.dir_r = dr;
                    g.dir_c = dc;
                }
            }
        } else {
            let mut possible_dirs = collect_possible_ghost_dirs(g.row, g.col, level);

            if possible_dirs.len() > 1 {
                possible_dirs.retain(|&(dr, dc)| !(dr == -g.dir_r && dc == -g.dir_c));
            }
            if !possible_dirs.is_empty() && rng.random::<f32>() < chase_prob {
                if let Some(chase_dir) = pick_best_chase_dir(
                    g.row,
                    g.col,
                    state.pac_row,
                    state.pac_col,
                    &possible_dirs,
                ) {
                    g.dir_r = chase_dir.0;
                    g.dir_c = chase_dir.1;
                }
            }
            g.row += g.dir_r;
            g.col += g.dir_c;
            wrap_ghost_coords(g);
        }
    }
}


pub fn check_collision(state: &mut GameState) {
    let mut collided_ghosts = Vec::new();

    for (i, g) in state.ghosts.iter().enumerate() {
        if g.row == state.pac_row && g.col == state.pac_col {
            collided_ghosts.push(i);
        }
    }
    if collided_ghosts.is_empty() {
        return;
    }

    if state.powerd_up {
        for i in collided_ghosts {

            state.score += state.ghosts_in_a_row * 20;

            state.ghosts_in_a_row *= 2;

            let (spawn_r, spawn_c) = state.ghost_spawns[i];
            reset_ghost(&mut state.ghosts[i], spawn_r, spawn_c);
        }
    } else {
        state.lives -= 1;
        if state.lives < 0 {
            return;
        }

        reset_pacman(state);
    }
}



fn is_wall(level: &[[Tile; COLS]; ROWS], row: i16, col: i16) -> bool {
    let (r, c) = wrap_coords(row, col);
    level[r as usize][c as usize] == Tile::Wall
}

fn collect_possible_ghost_dirs(
    ghost_r: i16,
    ghost_c: i16,
    level: &[[Tile; COLS]; ROWS],
) -> Vec<(i16, i16)> {
    let candidates = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut dirs = vec![];
    for &(dr, dc) in &candidates {
        let (wr, wc) = wrap_coords(ghost_r + dr, ghost_c + dc);
        if level[wr as usize][wc as usize] != Tile::Wall {
            dirs.push((dr, dc));
        }
    }
    dirs
}

fn pick_best_chase_dir(
    ghost_r: i16,
    ghost_c: i16,
    pac_r: i16,
    pac_c: i16,
    possible_dirs: &[(i16, i16)],
) -> Option<(i16, i16)> {
    let mut best_dir = None;
    let mut best_dist = i16::MAX;
    for &(dr, dc) in possible_dirs {
        let new_r = ghost_r + dr;
        let new_c = ghost_c + dc;
        let dist = (new_r - pac_r).abs() + (new_c - pac_c).abs();
        if dist < best_dist {
            best_dist = dist;
            best_dir = Some((dr, dc));
        }
    }
    best_dir
}

fn wrap_coords(row: i16, col: i16) -> (i16, i16) {
    let mut r = row;
    let mut c = col;
    if r < 0 {
        r = ROWS as i16 - 1;
    } else if r >= ROWS as i16 {
        r = 0;
    }
    if c < 0 {
        c = COLS as i16 - 1;
    } else if c >= COLS as i16 {
        c = 0;
    }
    (r, c)
}

fn wrap_ghost_coords(g: &mut Ghost) {
    if g.row < 0 {
        g.row = ROWS as i16 - 1;
    } else if g.row >= ROWS as i16 {
        g.row = 0;
    }
    if g.col < 0 {
        g.col = COLS as i16 - 1;
    } else if g.col >= COLS as i16 {
        g.col = 0;
    }
}

pub fn reset_ghost(ghost: &mut Ghost, spawn_r: i16, spawn_c: i16) {
    ghost.row = spawn_r;
    ghost.col = spawn_c;
    ghost.dir_r = 1;
    ghost.dir_c = 0;
}

pub fn reset_pacman(state: &mut GameState) {
    let (pr, pc) = state.pac_spawn;
    state.pac_row = pr;
    state.pac_col = pc;
    state.pac_dir_r = 0;
    state.pac_dir_c = -1;
    for (i, g) in state.ghosts.iter_mut().enumerate() {
        let (spawn_r, spawn_c) = state.ghost_spawns[i];
        reset_ghost(g, spawn_r, spawn_c);
    }
    state.powerd_up = false;
    state.ghosts_in_a_row = 0;
    state.power_end_time = None;
}
