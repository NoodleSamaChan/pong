#[cfg(test)]
mod test {
    use super::*;
    use pong::{rgb, creation_pongs, World, display};
    use insta::{assert_debug_snapshot, assert_snapshot};
    use std::time::Instant;
    use clap::{Parser, ValueEnum};
    use window_rs::WindowBuffer;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
    }

    #[test]
    fn pong_generation() {
        let cli = pong::Cli::parse();
        let mut buffer: WindowBuffer = WindowBuffer::new(5, 20);

        let mut game_elements: World = pong::World::new(
        Vec::new(),
        Vec::new(),
        0,
        0,
        (0, 0),
        false,
        Instant::now(),
        0,
        cli.ball_speed, 
        );

        creation_pongs(&mut game_elements, &buffer);
        display(&game_elements, &mut buffer, &cli);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        #....
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        .....
        .....
        .....
        .....
        .....
        .....
        .....
        .....
        .....
        "###
        );

    }
}
