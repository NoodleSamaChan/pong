use lib::{World, Cli};
use std::time::{Duration, Instant};
use window_rs::WindowBuffer;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use clap::{Parser, ValueEnum};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let mut buffer: WindowBuffer = WindowBuffer::new(cli.width, cli.height);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width(),
        buffer.height(),
        WindowOptions {
            scale: minifb::Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut game_elements: World = World::new(
        Vec::new(),
        Vec::new(),
        0,
        0,
        (0, 0),
        false,
        Instant::now(),
        0,
        cli.game_speed, 
    );

    let mut instant = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let _ = game_elements.handle_user_input(&window, &cli, &buffer);



        window
            .update_with_buffer(&buffer.buffer(), cli.width, cli.height)
            .unwrap();
    }
    Ok(())
}
