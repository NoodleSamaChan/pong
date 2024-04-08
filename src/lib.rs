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

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    Still,
}

#[derive(PartialEq)]
pub enum BallDirection {
    West,
    East,
    Still,
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
    pub file_path: Option<String>,
    #[arg(long, default_value_t = 120)]
    pub ball_speed: usize,
    #[arg(long, default_value_t = Difficulty::Medium)]
    pub difficulty: Difficulty,
    #[arg(long, default_value_t = false)]
    pub handicap: bool,
    #[arg(long, default_value_t = 10)]
    pub number_of_points_to_reach: usize,
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
    pub player_1_score: usize,
    pub player_2_score: usize,
    pub player_1_direction: Direction,
    pub player_2_direction: Direction,
    ball: Option<(usize, usize)>,
    ball_direction: BallDirection,
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
        player_2_score: usize,
        player_1_direction: Direction,
        player_2_direction: Direction,
        ball: Option<(usize, usize)>,
        ball_direction: BallDirection,
        finished: bool,
        small_break_timer: Instant,
        space_count: usize,
        game_speed: usize,
    ) -> Self {
        Self {
            player_1_pong,
            player_2_pong,
            player_1_score,
            player_2_score,
            player_1_direction,
            player_2_direction,
            ball,
            ball_direction,
            finished,
            small_break_timer,
            space_count,
            game_speed,
        }
    }

    pub fn reset(&mut self, buffer: &WindowBuffer) {
        self.player_1_score = 0;
        self.player_2_score = 0;
        creation_pongs(self, buffer);
        self.player_1_direction = Direction::Still;
        self.player_2_direction = Direction::Still;
        self.ball = Some((buffer.width() / 2, buffer.height() / 2));
        self.finished = false;
        self.space_count = self.space_count;
    }

    pub fn handle_user_input(
        &mut self,
        window: &Window,
        cli: &Cli,
        buffer: &WindowBuffer,
    ) -> std::io::Result<()> {
        if window.is_key_pressed(Key::Q, KeyRepeat::No) {
            self.reset(buffer);
        }

        if window.is_key_pressed(Key::E, KeyRepeat::Yes) {
            self.player_1_direction = Direction::North;
        }

        if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
            self.player_1_direction = Direction::South;
        }

        if window.is_key_pressed(Key::O, KeyRepeat::Yes) {
            self.player_2_direction = Direction::North;
        }

        if window.is_key_pressed(Key::K, KeyRepeat::Yes) {
            self.player_2_direction = Direction::South;
        }

        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            self.finished = false;
        }

        let small_break = Duration::from_millis(0);
        if self.small_break_timer.elapsed() >= small_break {
            window.get_keys_released().iter().for_each(|key| match key {
                Key::Space => self.space_count += 1,
                _ => (),
            });
            self.small_break_timer = Instant::now();
        }

        Ok(())
    }

    pub fn pong_1_direction(&mut self, buffer: &WindowBuffer) {
        let top = self.player_1_pong[0];
        let bottom = self.player_1_pong[self.player_1_pong.len() - 1];
        match self.player_1_direction {
            Direction::North => {
                if buffer.get(top.0 as isize, top.1 as isize - 1) != None {
                    println!("this is north");
                    self.player_1_pong.iter_mut().for_each(|(x, y)| *y -= 1);
                } else {
                    println!("I'm in the else");
                    self.player_1_direction = Direction::Still;
                    self.player_1_pong = self.player_1_pong.clone();
                }
            }
            Direction::South => {
                if buffer.get(bottom.0 as isize, bottom.1 as isize + 1) != None {
                    println!("this is south");
                    self.player_1_pong.iter_mut().for_each(|(x, y)| *y += 1);
                } else {
                    println!("I'm in the else");
                    self.player_1_direction = Direction::Still;
                    self.player_1_pong = self.player_1_pong.clone();
                }
            }
            Direction::Still => {
                self.player_1_pong = self.player_1_pong.clone();
                println!("I'm in the else of still");
            }
        }
    }

    pub fn pong_2_direction(&mut self, buffer: &WindowBuffer) {
        let top = self.player_2_pong[0];
        let bottom = self.player_2_pong[self.player_2_pong.len() - 1];
        match self.player_2_direction {
            Direction::North => {
                if buffer.get(top.0 as isize, top.1 as isize - 1) != None {
                    self.player_2_pong.iter_mut().for_each(|(x, y)| *y -= 1);
                } else {
                    self.player_2_pong = self.player_2_pong.clone();
                }
            }
            Direction::South => {
                if buffer.get(bottom.0 as isize, bottom.1 as isize + 1) != None {
                    self.player_2_pong.iter_mut().for_each(|(x, y)| *y += 1);
                } else {
                    self.player_2_pong = self.player_2_pong.clone();
                }
            }
            Direction::Still => {
                self.player_2_pong = self.player_2_pong.clone();
            }
        }
    }

    pub fn ball_movement_start(&mut self, buffer: &mut WindowBuffer) {
        let left_or_right = rand::thread_rng().gen_range(0..2);
        
        if let Some(ball) = &self.ball {
            let checker_first_pong = self.player_1_pong.iter().any(|(a, b)| (a, b) == (&ball.0, &ball.1));
            let checker_second_pong = self.player_2_pong.iter().any(|(a, b)| (a, b) == (&ball.0, &ball.1));
            if left_or_right == 0 {
                if buffer.get(ball.0 as isize - 1, ball.1 as isize) != None && checker_first_pong == false {
                    println!("this is north");
                    self.ball.iter_mut().for_each(|(x, y)| *x -= 1);
                } else if checker_first_pong == true {
                    self.ball_direction = BallDirection::East;
                }
            } else {
                if buffer.get(ball.0 as isize + 1, ball.1 as isize) != None && checker_first_pong == false {
                    println!("this is north");
                    self.ball.iter_mut().for_each(|(x, y)| *x += 1);
                } else if checker_first_pong == true {
                    self.ball_direction = BallDirection::West;
                }
            }
        }
    }

    pub fn update(&mut self, buffer: &mut WindowBuffer) {
        if self.space_count % 2 == 0 {
            self.pong_1_direction(buffer);
            self.pong_2_direction(buffer);
        }
    }
}

pub fn creation_pongs(world: &mut World, buffer: &WindowBuffer) {
    let y_middle_point = buffer.height() / 2;

    for x in 0..10 {
        world.player_1_pong.push((0, y_middle_point - x));
        world
            .player_2_pong
            .push((buffer.width() - 1, y_middle_point - x));
    }
}

pub fn display(world: &World, buffer: &mut WindowBuffer) {
    buffer.reset();
    world
        .player_1_pong
        .iter()
        .for_each(|(x, y)| buffer[(x.clone(), y.clone())] = rgb(0, 0, u8::MAX));

    world
        .player_2_pong
        .iter()
        .for_each(|(x, y)| buffer[(x.clone(), y.clone())] = rgb(0, u8::MAX, 0));

    if world.ball != None {
        if let Some(ball) = &world.ball {
            buffer[*ball] = rgb(u8::MAX, 0, 0);

            if (world.player_1_score as isize - world.player_2_score as isize == 2)
                || (world.player_1_score as isize - world.player_2_score as isize == (-2))
            {
                buffer[*ball] = rgb(100, 100, 0);
            } else if (world.player_1_score as isize - world.player_2_score as isize >= 4)
                || (world.player_1_score as isize - world.player_2_score as isize <= (-4))
            {
                buffer[*ball] = rgb(75, 75, 75);
            }
        }
    }
}
