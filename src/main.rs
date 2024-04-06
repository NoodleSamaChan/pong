use std::time::{Duration, Instant};
use window_rs::WindowBuffer;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use clap::{Parser, ValueEnum};
use pong::{creation_pongs, display, World};

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
        (buffer.width() / 2, buffer.height() / 2),
        false,
        Instant::now(),
        0,
        cli.ball_speed, 
    );

    let mut instant = Instant::now();
    creation_pongs(&mut game_elements, &buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        while game_elements.player_1_score <= cli.number_of_points_to_reach || game_elements.player_2_score <= cli.number_of_points_to_reach {
            let _ = game_elements.handle_user_input(&window, &cli, &buffer);
            display(&game_elements, &mut buffer);



            window
                .update_with_buffer(&buffer.buffer(), buffer.width(), buffer.height())
                .unwrap();

        }
    }
    Ok(())
}
