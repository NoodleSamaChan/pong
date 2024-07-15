use clap::{Parser, ValueEnum};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use pong::{creation_pongs, display, rgb, World};
use rand::rngs::StdRng;
use rand::SeedableRng;
use window_rs::WindowBuffer;
use graphic::{minifb::Minifb, Graphic};
use web_time::Instant;

fn main() -> std::io::Result<()> {
    let cli = pong::Cli::parse();

    let mut buffer: WindowBuffer = WindowBuffer::new(90, 60);

    let mut window = Minifb::new("Pong - ESC to exit", buffer.width(), buffer.height());

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
        cli.pong_speed,
        cli.ball_speed,
        StdRng::seed_from_u64(75),
        0x00FF0000,
        0xFF00FF00,
        0xFFFFFF00,
    );

    let mut instant_ball = Instant::now();
    let mut instant_pong = Instant::now();
    creation_pongs(&mut game_elements, &buffer);

    while window.is_open() && !window.is_key_down(graphic::Key::Escape) {
        while game_elements.finished == false
        {
            let _ = game_elements.handle_user_input(&window, &buffer);
            game_elements.update(&mut buffer, &cli, &mut instant_pong, &mut instant_ball);
            display(&game_elements, &mut buffer);

            window
                .update_with_buffer(&buffer)
        }

        println!(
            "Game over! Score player 1 is {}, score player 2 is {}",
            game_elements.player_1_score, game_elements.player_2_score
        );

        window
            .update_with_buffer(&buffer)

    }
    Ok(())
}
