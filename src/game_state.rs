use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub const ROWS: usize = 29;
pub const COLS: usize = 28;
pub const SPEED_OF_GAME: u64 = 175;
pub const HOW_SLOW: u8 = 3;
pub const INITIAL_LIVES: i32 = 3;
pub const INITIAL_FREE_LIFE: i32 = 1000;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Pellet = 2,
    PowerUp = 3,
    GhostWall = 4,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ghost {
    pub row: i16,
    pub col: i16,
    pub dir_r: i16, // -1 up, +1 down
    pub dir_c: i16, // -1 left, +1 right
}

pub struct GameState {
    pub level: [[Tile; COLS]; ROWS],
    pub pac_row: i16,
    pub pac_col: i16,
    pub pac_dir_r: i16,
    pub pac_dir_c: i16,
    pub pac_spawn: (i16, i16),
    pub ghosts: [Ghost; 4],
    pub ghost_spawns: [(i16, i16); 4],
    pub score: i32,
    pub lives: i32,
    pub free_life: i32,
    pub level_number: i32,
    pub powerd_up: bool,
    pub ghosts_in_a_row: i32,
    pub power_end_time: Option<Instant>,
    pub food_left: i32,
    pub slower_ghosts_counter: u8,
    pub chase_probability: f32,
}

impl GameState {
    pub fn new() -> Self {
        let level = [[Tile::Empty; COLS]; ROWS];
        let ghosts = [
            Ghost {
                row: 0,
                col: 0,
                dir_r: 1,
                dir_c: 0,
            },
            Ghost {
                row: 0,
                col: 0,
                dir_r: -1,
                dir_c: 0,
            },
            Ghost {
                row: 0,
                col: 0,
                dir_r: 0,
                dir_c: -1,
            },
            Ghost {
                row: 0,
                col: 0,
                dir_r: 0,
                dir_c: 1,
            },
        ];
        Self {
            level,
            pac_row: 0,
            pac_col: 0,
            pac_dir_r: 0,
            pac_dir_c: -1,
            pac_spawn: (0, 0),
            ghosts,
            ghost_spawns: [(0, 0); 4],
            score: 0,
            lives: INITIAL_LIVES,
            free_life: INITIAL_FREE_LIFE,
            level_number: 1,
            powerd_up: false,
            ghosts_in_a_row: 0,
            power_end_time: None,
            food_left: 0,
            slower_ghosts_counter: 0,
            chase_probability: 0.5,
        }
    }

    pub fn load_level_from_file(&mut self, path: &str) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        if path.contains("level09") {
            self.level_number = 9;
            self.chase_probability = 0.95;
        } else if path.contains("level08") {
            self.level_number = 8;
            self.chase_probability = 0.85;
        } else if path.contains("level07") {
            self.level_number = 7;
            self.chase_probability = 0.75;
        } else if path.contains("level06") {
            self.level_number = 6;
            self.chase_probability = 0.70;
        } else if path.contains("level05") {
            self.level_number = 5;
            self.chase_probability = 0.65;
        } else if path.contains("level04") {
            self.level_number = 4;
            self.chase_probability = 0.60;
        } else if path.contains("level03") {
            self.level_number = 3;
            self.chase_probability = 0.55;
        } else if path.contains("level02") {
            self.level_number = 2;
            self.chase_probability = 0.5;
        } else {
            self.level_number = 1;
            self.chase_probability = 0.5;
        }

        let mut row_idx = 0;
        for line in reader.lines().take(ROWS) {
            let line = line?;
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap_or(0))
                .collect();
            for (col_idx, &val) in nums.iter().take(COLS).enumerate() {
                let tile = match val {
                    1 => Tile::Wall,
                    2 => Tile::Pellet,
                    3 => Tile::PowerUp,
                    4 => Tile::GhostWall,
                    5 => {
                        self.ghosts[0].row = row_idx as i16;
                        self.ghosts[0].col = col_idx as i16;
                        self.ghost_spawns[0] = (row_idx as i16, col_idx as i16);
                        Tile::Empty
                    }
                    6 => {
                        self.ghosts[1].row = row_idx as i16;
                        self.ghosts[1].col = col_idx as i16;
                        self.ghost_spawns[1] = (row_idx as i16, col_idx as i16);
                        Tile::Empty
                    }
                    7 => {
                        self.ghosts[2].row = row_idx as i16;
                        self.ghosts[2].col = col_idx as i16;
                        self.ghost_spawns[2] = (row_idx as i16, col_idx as i16);
                        Tile::Empty
                    }
                    8 => {
                        self.ghosts[3].row = row_idx as i16;
                        self.ghosts[3].col = col_idx as i16;
                        self.ghost_spawns[3] = (row_idx as i16, col_idx as i16);
                        Tile::Empty
                    }
                    9 => {
                        self.pac_row = row_idx as i16;
                        self.pac_col = col_idx as i16;
                        self.pac_spawn = (row_idx as i16, col_idx as i16);
                        Tile::Empty
                    }
                    _ => Tile::Empty,
                };
                self.level[row_idx][col_idx] = tile;
            }
            row_idx += 1;
        }

        // Count food remain
        let mut food_count = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                if self.level[r][c] == Tile::Pellet {
                    food_count += 1;
                }
            }
        }
        self.food_left = food_count;

        Ok(())
    }
}
