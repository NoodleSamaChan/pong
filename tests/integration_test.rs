#[cfg(test)]
mod test {
    use super::*;
    use pong::Cli;
    use pong::{rgb, creation_pongs, World, display};
    use insta::{assert_debug_snapshot, assert_snapshot};
    use std::time::Instant;
    use clap::{Parser, ValueEnum};
    use window_rs::WindowBuffer;
    use rand::rngs::ThreadRng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_rgb() {
        assert_eq!(rgb(0, 0, 0), 0x00_00_00_00);
        assert_eq!(rgb(255, 255, 255), 0x00_ff_ff_ff);
        assert_eq!(rgb(0x12, 0x34, 0x56), 0x00_12_34_56);
    }

    #[test]
    fn pong_generation() {
        let mut buffer: WindowBuffer = WindowBuffer::new(5, 20);

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
        120, 
        StdRng::seed_from_u64(75),
        );

        creation_pongs(&mut game_elements, &buffer);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #.#.#
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

    #[test]
    fn pongs_movements() {
        let cli = pong::Cli::parse();
        let mut buffer: WindowBuffer = WindowBuffer::new(5, 25);

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
        120, 
        StdRng::seed_from_u64(75),
        );

        creation_pongs(&mut game_elements, &buffer);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        .....
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #.#.#
        .....
        .....
        .....
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

        game_elements.player_1_direction = pong::Direction::North;
        game_elements.player_2_direction = pong::Direction::South;
        game_elements.update(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        .....
        #....
        #....
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        ..#.#
        ....#
        .....
        .....
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

        game_elements.update(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        .....
        #....
        #....
        #....
        #....
        #...#
        #...#
        #...#
        #...#
        #...#
        #...#
        ....#
        ..#.#
        ....#
        ....#
        .....
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

        game_elements.update(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        #....
        #....
        #....
        #....
        #....
        #....
        #...#
        #...#
        #...#
        #...#
        ....#
        ....#
        ..#.#
        ....#
        ....#
        ....#
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

        #[test]
    fn ball_launch_right() {
        let cli = pong::Cli::parse();
        let mut buffer: WindowBuffer = WindowBuffer::new(15, 10);

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
        120, 
        StdRng::seed_from_u64(75),
        );

        game_elements.ball_movement(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ...............
        ...............
        ...............
        ...............
        ...............
        ........#......
        ...............
        ...............
        ...............
        ...............
        "###
        );

        game_elements.ball_movement(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ...............
        ...............
        ...............
        ...............
        ...............
        .........#.....
        ...............
        ...............
        ...............
        ...............
        "###
        );

        game_elements.ball_movement(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ...............
        ...............
        ...............
        ...............
        ...............
        ..........#....
        ...............
        ...............
        ...............
        ...............
        "###
        );

        game_elements.ball_movement(&mut buffer, &cli);
        display(&game_elements, &mut buffer);

        assert_snapshot!(
            buffer.to_string(),
            @r###"
        ...............
        ...............
        ...............
        ...............
        ...............
        ...........#...
        ...............
        ...............
        ...............
        ...............
        "###
        );

    }
    
}
