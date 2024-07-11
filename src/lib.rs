use clap::{Parser, ValueEnum};
use rand::rngs::StdRng;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use window_rs::WindowBuffer;
use graphic::{Graphic, Key};

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
    NorthWest,
    SouthWest,
    East,
    NorthEast,
    SouthEast,
    Launch,
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

    //#[arg(long)]
    //pub file_path: Option<String>,
    #[arg(long, default_value_t = 20)]
    pub ball_speed: usize,
    #[arg(long, default_value_t = 60)]
    pub pong_speed: usize,
    #[arg(long, default_value_t = Difficulty::Medium)]
    pub difficulty: Difficulty,
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
    pub player_1_pong: Vec<(usize, usize)>,
    pub player_2_pong: Vec<(usize, usize)>,
    pub player_1_score: usize,
    pub player_2_score: usize,
    pub player_1_direction: Direction,
    pub player_2_direction: Direction,
    ball: Option<(usize, usize)>,
    ball_direction: BallDirection,
    pub finished: bool,
    small_break_timer: Instant,
    space_count: usize,
    pong_speed: usize,
    ball_speed: usize,
    rng: StdRng,
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
        pong_speed: usize,
        ball_speed: usize,
        rng: StdRng,
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
            pong_speed,
            ball_speed,
            rng,
        }
    }

    pub fn reset(&mut self, buffer: &WindowBuffer) {
        self.player_1_pong = Vec::new();
        self.player_2_pong = Vec::new();
        self.player_1_score = 0;
        self.player_2_score = 0;
        creation_pongs(self, buffer);
        self.player_1_direction = Direction::Still;
        self.player_2_direction = Direction::Still;
        self.ball = Some((buffer.width() / 2, buffer.height() / 2));
        self.finished = false;
        self.space_count = self.space_count;
        self.ball_direction = BallDirection::Still;
    }

    pub fn handle_user_input <W: Graphic>(
        &mut self,
        window: &W,
        buffer: &WindowBuffer,
    ) -> std::io::Result<()> {
        if window.is_key_pressed(Key::Quit) {
            self.reset(buffer);
        }

        if window.is_key_pressed(Key::UpPlayer1) {
            self.player_1_direction = Direction::North;
        }

        if window.is_key_pressed(Key::DownPlayer1) {
            self.player_1_direction = Direction::South;
        }

        if window.is_key_pressed(Key::UpPlayer2) {
            self.player_2_direction = Direction::North;
        }

        if window.is_key_pressed(Key::DownPlayer2) {
            self.player_2_direction = Direction::South;
        }

        if window.is_key_pressed(Key::Launch) {
            if self.ball_direction == BallDirection::Still {
                self.ball_direction = BallDirection::Launch
            }
        }

        let small_break = Duration::from_millis(0);
        if self.small_break_timer.elapsed() >= small_break {
            window.get_keys_released().iter().for_each(|key| match key {
                graphic::Key::Space => self.space_count += 1,
                _ => (),
            });
            self.small_break_timer = Instant::now();
        }

        Ok(())
    }

    pub fn pong_1_direction(&mut self, buffer: &WindowBuffer) {
        let top = self.player_1_pong[self.player_1_pong.len() - 1];
        let bottom = self.player_1_pong[0];
        match self.player_1_direction {
            Direction::North => {
                if buffer.get(top.0 as isize, top.1 as isize - 2) != None {
                    self.player_1_pong.iter_mut().for_each(|(x, y)| *y -= 1);
                } else {
                    self.player_1_direction = Direction::Still;
                    self.player_1_pong = self.player_1_pong.clone();
                }
            }
            Direction::South => {
                if buffer.get(bottom.0 as isize, bottom.1 as isize + 2) != None {
                    self.player_1_pong.iter_mut().for_each(|(x, y)| *y += 1);
                } else {
                    self.player_1_direction = Direction::Still;
                    self.player_1_pong = self.player_1_pong.clone();
                }
            }
            Direction::Still => {
                self.player_1_pong = self.player_1_pong.clone();
            }
        }
        self.player_1_direction = Direction::Still;
    }

    pub fn pong_2_direction(&mut self, buffer: &WindowBuffer) {
        let top = self.player_2_pong[self.player_2_pong.len() - 1];
        let bottom = self.player_2_pong[0];
        match self.player_2_direction {
            Direction::North => {
                if buffer.get(top.0 as isize, top.1 as isize - 2) != None {
                    self.player_2_pong.iter_mut().for_each(|(x, y)| *y -= 1);
                } else {
                    self.player_2_pong = self.player_2_pong.clone();
                }
            }
            Direction::South => {
                if buffer.get(bottom.0 as isize, bottom.1 as isize + 2) != None {
                    self.player_2_pong.iter_mut().for_each(|(x, y)| *y += 1);
                } else {
                    self.player_2_pong = self.player_2_pong.clone();
                }
            }
            Direction::Still => {
                self.player_2_pong = self.player_2_pong.clone();
            }
        }
        self.player_2_direction = Direction::Still;
    }

    pub fn ball_movement(&mut self, buffer: &mut WindowBuffer, cli: &Cli) {
        if let Some(ball) = &self.ball {
            let left_or_right = self.rng.gen_range(0..2);
            let ball_rebounce_direction = self.rng.gen_range(0..3);
            let checker_first_pong = self
                .player_1_pong
                .iter()
                .any(|(a, b)| (a, b) == (&ball.0, &ball.1));
            let checker_second_pong = self
                .player_2_pong
                .iter()
                .any(|(a, b)| (a, b) == (&ball.0, &ball.1));
            match self.ball_direction {
                BallDirection::West => {
                    if buffer.get(ball.0 as isize - 1, ball.1 as isize) != None
                        && checker_first_pong == false
                    {
                        self.ball = Some((ball.0 - 1, ball.1));
                    } else if checker_first_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::East;
                        } else if ball_rebounce_direction == 1 {
                            self.ball_direction = BallDirection::NorthEast;
                        } else {
                            self.ball_direction = BallDirection::SouthEast;
                        }
                    } else if ball == &(0, ball.1) {
                        self.player_2_score += 1;
                        creation_ball(self, buffer, cli)
                    }
                }
                BallDirection::NorthWest => {
                    if buffer.get(ball.0 as isize - 1, ball.1 as isize - 1) != None
                        && checker_first_pong == false
                    {
                        self.ball = Some((ball.0 - 1, ball.1 - 1));
                    } else if ball.1 == 0
                        && buffer.get(ball.0 as isize - 1, ball.1 as isize + 1) != None
                        && checker_first_pong == false
                    {
                        self.ball = Some((ball.0 - 1, ball.1 + 1));
                        self.ball_direction = BallDirection::SouthWest;
                    } else if checker_first_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::East;
                        } else {
                            self.ball_direction = BallDirection::SouthEast;
                        }
                    } else if ball == &(0, ball.1) {
                        self.player_2_score += 1;
                        creation_ball(self, buffer, cli)
                    }
                }
                BallDirection::SouthWest => {
                    if buffer.get(ball.0 as isize - 1, ball.1 as isize + 1) != None
                        && checker_first_pong == false
                    {
                        self.ball = Some((ball.0 - 1, ball.1 + 1));
                    } else if ball.1 == buffer.height() - 1
                        && buffer.get(ball.0 as isize - 1, ball.1 as isize - 1) != None
                        && checker_first_pong == false
                    {
                        self.ball = Some((ball.0 - 1, ball.1 - 1));
                        self.ball_direction = BallDirection::NorthWest;
                    } else if checker_first_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::East;
                        } else {
                            self.ball_direction = BallDirection::NorthEast;
                        }
                    } else if ball == &(0, ball.1) {
                        self.player_2_score += 1;
                        creation_ball(self, buffer, cli)
                    }
                }
                BallDirection::East => {
                    if buffer.get(ball.0 as isize + 1, ball.1 as isize) != None
                        && checker_second_pong == false
                    {
                        self.ball = Some((ball.0 + 1, ball.1));
                    } else if checker_second_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::West;
                        } else if ball_rebounce_direction == 1 {
                            self.ball_direction = BallDirection::NorthWest;
                        } else {
                            self.ball_direction = BallDirection::SouthWest;
                        }
                    } else if ball == &(buffer.width() - 1, ball.1) {
                        self.player_1_score += 1;
                        self.ball = None;
                        creation_ball(self, buffer, cli);
                    }
                }
                BallDirection::NorthEast => {
                    if buffer.get(ball.0 as isize + 1, ball.1 as isize - 1) != None
                        && checker_second_pong == false
                    {
                        self.ball = Some((ball.0 + 1, ball.1 - 1));
                    } else if ball.1 == 0
                        && buffer.get(ball.0 as isize + 1, ball.1 as isize + 1) != None
                        && checker_second_pong == false
                    {
                        self.ball = Some((ball.0 + 1, ball.1 + 1));
                        self.ball_direction = BallDirection::SouthEast;
                    } else if checker_second_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::West;
                        } else {
                            self.ball_direction = BallDirection::SouthWest;
                        }
                    } else if ball == &(buffer.width() - 1, ball.1) {
                        self.player_1_score += 1;
                        creation_ball(self, buffer, cli);
                    }
                }
                BallDirection::SouthEast => {
                    if buffer.get(ball.0 as isize + 1, ball.1 as isize + 1) != None
                        && checker_second_pong == false
                    {
                        self.ball = Some((ball.0 + 1, ball.1 + 1));
                    } else if ball.1 == buffer.height() - 1
                        && buffer.get(ball.0 as isize + 1, ball.1 as isize - 1) != None
                        && checker_second_pong == false
                    {
                        self.ball = Some((ball.0 + 1, ball.1 - 1));
                        self.ball_direction = BallDirection::NorthEast;
                    } else if checker_second_pong == true {
                        if ball_rebounce_direction == 0 {
                            self.ball_direction = BallDirection::West;
                        } else {
                            self.ball_direction = BallDirection::NorthWest;
                        }
                    } else if ball == &(buffer.width() - 1, ball.1) {
                        self.player_1_score += 1;
                        creation_ball(self, buffer, cli);
                    }
                }
                BallDirection::Launch => {
                    if left_or_right == 0 {
                        if buffer.get(ball.0 as isize - 1, ball.1 as isize) != None
                            && checker_first_pong == false
                        {
                            self.ball = Some((ball.0 - 1, ball.1));
                            self.ball_direction = BallDirection::West;
                        }
                    } else {
                        if buffer.get(ball.0 as isize + 1, ball.1 as isize) != None
                            && checker_second_pong == false
                        {
                            self.ball = Some((ball.0 + 1, ball.1));
                            self.ball_direction = BallDirection::East;
                        }
                    }
                }
                BallDirection::Still => {
                    self.ball = self.ball;
                }
            }
        }
    }

    pub fn update(&mut self, buffer: &mut WindowBuffer, cli: &Cli, pong_time: &mut Instant, ball_time: &mut Instant) {

        let elapsed_time_ball = Duration::from_millis(cli.ball_speed as u64);
        let elapsed_time_pongs = Duration::from_millis(cli.pong_speed as u64);
        if self.space_count % 2 == 0 {

            if pong_time.elapsed() >= elapsed_time_pongs {
                self.pong_1_direction(buffer);
                self.pong_2_direction(buffer);
                *pong_time = Instant::now();
            }
            if ball_time.elapsed() >= elapsed_time_ball {
                self.ball_movement(buffer, cli);
                *ball_time = Instant::now();
            }   
        }
    }
}

pub fn creation_ball(world: &mut World, buffer: &WindowBuffer, cli: &Cli) {
    world.ball = None;
    if world.player_1_score < cli.number_of_points_to_reach
        && world.player_2_score < cli.number_of_points_to_reach
    {
        world.ball = Some((buffer.width() / 2, buffer.height() / 2));
        world.ball_direction = BallDirection::Still;
    } else {
        world.finished = true;
        println!(
            "Game over! Score player 1 is {}, score player 2 is {}",
            world.player_1_score, world.player_2_score
        );
        world.reset(&buffer);
    }
}

pub fn creation_pongs(world: &mut World, buffer: &WindowBuffer) {
    let y_middle_point = buffer.height() / 2;

    for x in 0..5 {
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
                buffer[*ball] = rgb(200, 80, 80);
            } else if (world.player_1_score as isize - world.player_2_score as isize == 3)
                || (world.player_1_score as isize - world.player_2_score as isize == (-3))
            {
                buffer[*ball] = rgb(150, 100, 100);
            } else if (world.player_1_score as isize - world.player_2_score as isize == 4)
                || (world.player_1_score as isize - world.player_2_score as isize == (-4))
            {
                buffer[*ball] = rgb(150, 150, 150);
            } else if (world.player_1_score as isize - world.player_2_score as isize >= 5)
                || (world.player_1_score as isize - world.player_2_score as isize <= (-5))
            {
                buffer[*ball] = rgb(200, 200, 200);
            } 
        }
    }
}
