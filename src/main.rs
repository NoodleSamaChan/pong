use clap::{Parser, ValueEnum};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use pong::{creation_pongs, display, World};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::time::{Duration, Instant};
use window_rs::WindowBuffer;

fn main() -> std::io::Result<()> {
    let cli = pong::Cli::parse();

    let mut buffer: WindowBuffer = WindowBuffer::new(140, 110);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width(),
        buffer.height(),
        WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut game_elements: World = pong::World::new(
        Vec::new(),
        Vec::new(),
        0,
        0,
        pong::Direction::Still,
        pong::Direction::Still,
        Some((buffer.width() / 2, buffer.height() / 2)),
        pong::BallDirection::Still,
        false,
        Instant::now(),
        0,
        cli.ball_speed,
        StdRng::seed_from_u64(75),
    );

    let mut instant = Instant::now();
    creation_pongs(&mut game_elements, &buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        while game_elements.finished == false
        {
            let _ = game_elements.handle_user_input(&window, &buffer);
            game_elements.update(&mut buffer, &cli);
            display(&game_elements, &mut buffer);

            window
                .update_with_buffer(&buffer.buffer(), buffer.width(), buffer.height())
                .unwrap();
        }

        println!(
            "Game over! Score player 1 is {}, score player 2 is {}",
            game_elements.player_1_score, game_elements.player_2_score
        );

    }
    Ok(())
}
