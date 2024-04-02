use clap::{Parser, ValueEnum};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use window_rs::WindowBuffer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Medium => write!(f, "medium"),
            Difficulty::Hard => write!(f, "hard"),
            &Difficulty::Easy => write!(f, "easy"),
        }
    }
}

//CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on

    #[arg(long)]
    file_path: Option<String>,
    #[arg(long, default_value_t = 120)]
    ball_speed: usize,
    #[arg(long, default_value_t = Difficulty::Medium)]
    difficulty: Difficulty,
    #[arg(long, default_value_t = false)]
    handicap: bool,
}
//CLI END

//COLOURS MANAGEMENT
pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
    let a = u32::from(red);
    let b: u32 = u32::from(green);
    let c = u32::from(blue);

    let new_red = a << 16;
    let new_green = b << 8;

    let final_number = new_red | new_green | c;

    return final_number;
}
//COLOURS MANAGEMENT END

pub struct World {
    player_1_pong: Vec<(usize, usize)>,
    player_2_pong: Vec<(usize, usize)>,
    player_1_score: usize,
    player_1_score: usize,
    ball: (usize, usize),
    finished: bool,
    small_break_timer: Instant,
    space_count: usize,
    game_speed: usize,
}

impl World {
    pub fn new(
        player_1_pong: Vec<(usize, usize)>,
        player_2_pong: Vec<(usize, usize)>,
        player_1_score: usize,
        player_1_score: usize,
        ball: (usize, usize),
        finished: bool,
        small_break_timer: Instant,
        space_count: usize,
        game_speed: usize,
    ) -> Self {
        Self {
            player_1_pong,
            player_2_pong,
            player_1_score,
            player_1_score,
            ball,
            finished,
            small_break_timer,
            space_count,
            game_speed,
        }
    }
}

pub fn creation_pongs (world: &mut World, buffer: WindowBuffer) {
    let y_middle_point = buffer.height() / 2;

    world.player_1_pong.push((0, y_middle_point));
    world.player_1_pong.push((0, y_middle_point - 1));
    world.player_1_pong.push((0, y_middle_point - 2));
    world.player_1_pong.push((0, y_middle_point - 3));
    world.player_1_pong.push((0, y_middle_point - 4));

    world.player_2_pong.push((buffer.width(), y_middle_point));
    world.player_2_pong.push((buffer.width(), y_middle_point - 1));
    world.player_2_pong.push((buffer.width(), y_middle_point - 2));
    world.player_2_pong.push((buffer.width(), y_middle_point - 3));
    world.player_2_pong.push((buffer.width(), y_middle_point - 4));
    
}
