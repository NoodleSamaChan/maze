use rand::prelude::SliceRandom;
use rand::Rng;
use window_rs::WindowBuffer;
use graphic::{self, Graphic, Key};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MazeConfig {
    /// The color used for the path
    pub path_color: u32,

    /// The color used for the walls
    pub wall_color: u32,

    /// Quantity of open walls in the maze
    pub open_walls: usize,
}

impl Default for MazeConfig {
    fn default() -> Self {
        Self {
            path_color: 0,
            wall_color: u32::MAX,
            open_walls: 0,
        }
    }
}

impl MazeConfig {
    pub fn generate(&self, buffer: &mut WindowBuffer, rng: &mut impl Rng) {
        buffer.fill(self.wall_color);

        let x = rng.gen_range(0..buffer.width());
        let y = rng.gen_range(0..buffer.height());

        let mut stack = Vec::new();
        stack.push((x, y));
        buffer[(x, y)] = self.path_color;

        while let Some(cell) = stack.pop() {
            let (x, y) = (cell.0 as isize, cell.1 as isize);

            let mut to_explore = Vec::new();

            if let Some(cell) = buffer.get(x - 2, y) {
                if cell == self.wall_color {
                    let (x, y) = (x as usize, y as usize);
                    to_explore.push((x as usize - 2, y as usize));
                }
            }
            if let Some(cell) = buffer.get(x + 2, y) {
                if cell == self.wall_color {
                    let (x, y) = (x as usize, y as usize);
                    to_explore.push((x as usize + 2, y as usize));
                }
            }
            if let Some(cell) = buffer.get(x, y - 2) {
                if cell == self.wall_color {
                    let (x, y) = (x as usize, y as usize);
                    to_explore.push((x as usize, y as usize - 2));
                }
            }
            if let Some(cell) = buffer.get(x, y + 2) {
                if cell == self.wall_color {
                    let (x, y) = (x as usize, y as usize);
                    to_explore.push((x as usize, y as usize + 2));
                }
            }

            if let Some(cell2) = to_explore.choose(rng) {
                if to_explore.len() > 1 {
                    stack.push(cell);
                }
                stack.push(*cell2);
                buffer[*cell2] = self.path_color;
                let wall = middle_point(cell, *cell2);
                buffer[wall] = self.path_color;
            }
        }

        // Most of the maze has been generated, we need to fix the edges
        // We're going to randomely convert some walls on the edge into pathway
        let width = buffer.width();
        let height = buffer.height();
        // Fix the top and bottom
        for x in 1..(width - 1) {
            // we don't want to create an ugly pattern of wall so we're going to randomely break some walls
            if buffer[(x, 0)] == self.wall_color
                && buffer[(x - 1, 0)] == self.wall_color
                && buffer[(x + 1, 0)] == self.wall_color
                && buffer[(x, 1)] == self.path_color
                && rng.gen()
            {
                buffer[(x, 0)] = self.path_color;
            }
            if buffer[(x, height - 1)] == self.wall_color
                && buffer[(x - 1, height - 1)] == self.wall_color
                && buffer[(x + 1, height - 1)] == self.wall_color
                && buffer[(x, height - 2)] == self.path_color
                && rng.gen()
            {
                buffer[(x, height - 1)] = self.path_color;
            }
        }
        for y in 1..(height - 1) {
            // we don't want to create an ugly pattern of wall so we're going to randomely break some walls
            if buffer[(0, y)] == self.wall_color
                && buffer[(0, y - 1)] == self.wall_color
                && buffer[(0, y + 1)] == self.wall_color
                && buffer[(1, y)] == self.path_color
                && rng.gen()
            {
                buffer[(0, y)] = self.path_color;
            }
            if buffer[(width - 1, y)] == self.wall_color
                && buffer[(width - 1, y - 1)] == self.wall_color
                && buffer[(width - 1, y + 1)] == self.wall_color
                && buffer[(width - 2, y)] == self.path_color
                && rng.gen()
            {
                buffer[(width - 1, y)] = self.path_color;
            }
        }

        // Now let's open some walls

        // 1. We starts by opening interesting walls first
        //    If we don't find any interesting wall for 100 try we exit
        let mut opened_walls = 0;
        let mut retry = 0;
        while opened_walls < self.open_walls && retry < 100 {
            let x = rng.gen_range(1..buffer.width() - 1);
            let y = rng.gen_range(1..buffer.height() - 1);
            if buffer[(x, y)] == self.wall_color && self.path_around((x, y), &buffer) == 2 {
                opened_walls += 1;
                buffer[(x, y)] = self.path_color;
                // We found an interesting wall, we can reset the number of try
                retry = 0;
            }
            // We didn't find anything
            retry += 1;
        }

        // 2. We can't find any interesting walls anymore, we're just going to drop all remaining walls
        'outer: for x in 0..buffer.width() {
            for y in 0..buffer.height() {
                if buffer[(x, y)] == self.wall_color {
                    buffer[(x, y)] = self.path_color;
                    opened_walls += 1;
                    if opened_walls > self.open_walls {
                        break 'outer;
                    }
                }
            }
        }
    }

    fn path_around(&self, (x, y): (usize, usize), buffer: &WindowBuffer) -> usize {
        let (x, y) = (x as isize, y as isize);

        buffer
            .get(x - 1, y)
            .map_or(false, |cell| cell == self.path_color) as usize
            + buffer
                .get(x + 1, y)
                .map_or(false, |cell| cell == self.path_color) as usize
            + buffer
                .get(x, y - 1)
                .map_or(false, |cell| cell == self.path_color) as usize
            + buffer
                .get(x, y + 1)
                .map_or(false, |cell| cell == self.path_color) as usize
    }
}

pub fn start_end_generator(
    buffer: &mut WindowBuffer,
    rng: &mut impl Rng,
    player: &mut Player,
) -> (usize, usize) {
    let mut start_point_ready = false;
    let mut end_point_ready = false;
    loop {
        let start_height = rng.gen_range(0..buffer.height());
        let end_height = rng.gen_range(0..buffer.height());
        let width_max = buffer.width() - 1;

        if start_point_ready == false && buffer[(1, start_height)] == 0 {
            buffer[(0, start_height)] = player.player_color;
            start_point_ready = true;
            println!("start point");
        }
        if end_point_ready == false && buffer[(&width_max - 1, end_height)] == 0 {
            buffer[(width_max, end_height)] = player.finish_color;
            end_point_ready = true;
            println!("end point");
        }
        if start_point_ready == true && end_point_ready == true {
            player.end_point = (width_max, end_height);
            player.position = (0, start_height);
            player.previous_spot = (0, start_height);
            return (0, start_height);
        }
    }
}

fn middle_point(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    if a.0 != b.0 {
        let min = a.0.min(b.0);
        let max = a.0.max(b.0);

        (min + (max - min) / 2, a.1)
    } else if a.1 != b.1 {
        let min = a.1.min(b.1);
        let max = a.1.max(b.1);

        (a.0, min + (max - min) / 2)
    } else {
        a
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Still,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Player {
    pub player_color: u32,
    pub position: (usize, usize),

    pub finish_color: u32,
    pub end_point: (usize, usize),

    pub direction: Direction,
    pub previous_spot: (usize, usize),
    pub maze_config: MazeConfig,

    pub game_over: bool,
}

impl Player {
    pub fn new(
        position: (usize, usize),
        end_point: (usize, usize),
        direction: Direction,
        previous_spot: (usize, usize),
        maze_config: MazeConfig,
        game_over: bool,
    ) -> Self {
        Self {
            player_color: 0x00FF0000,
            finish_color: 0xFF00FF00,
            position,
            end_point,
            direction,
            previous_spot,
            maze_config,
            game_over,
        }
    }

    pub fn handle_user_input<W: Graphic>(
        &mut self,
        window: &W,
        start_point: &(usize, usize),
    ) -> std::io::Result<()> {
        if window.is_key_pressed(Key::Quit) {
            self.reset(*start_point);
        }

        if window.is_key_pressed(Key::Up) {
            self.direction = Direction::North;
        }

        if window.is_key_pressed(Key::Down) {
            self.direction = Direction::South;
        }

        if window.is_key_pressed(Key::Right) {
            self.direction = Direction::East;
        }

        if window.is_key_pressed(Key::Left) {
            self.direction = Direction::West;
        }

        /*let small_break = Duration::from_millis(0);
        if self.small_break_timer.elapsed() >= small_break {
            window.get_keys_released().iter().for_each(|key| match key {
                Key::Space => self.space_count += 1,
                _ => (),
            });
            self.small_break_timer = Instant::now();
        }*/

        Ok(())
    }

    pub fn reset(&mut self, start_point: (usize, usize)) {
        self.position = start_point;
        self.direction = Direction::Still;
    }

    pub fn direction(&mut self, buffer: &WindowBuffer) {
        let x = self.position.0;
        let y = self.position.1;
        match self.direction {
            Direction::East => {
                if buffer.get(x as isize + 1, y as isize) != None
                    && buffer[(x + 1, y)] != self.maze_config.wall_color
                {
                    if (x + 1, y) == self.end_point {
                        println!("Congrats, you've finished the maze!");
                        self.position = (x + 1, y);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                        self.game_over = true;
                    } else {
                        self.position = (x + 1, y);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                    }
                }
            }
            Direction::North => {
                if buffer.get(x as isize, y as isize - 1) != None
                    && buffer[(x, y - 1)] != self.maze_config.wall_color
                {
                    if (x, y - 1) == self.end_point {
                        println!("Congrats, you've finished the maze!");
                        self.position = (x, y - 1);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                        self.game_over = true;
                    } else {
                        self.position = (x, y - 1);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                    }
                }
            }
            Direction::South => {
                if buffer.get(x as isize, y as isize + 1) != None
                    && buffer[(x, y + 1)] != self.maze_config.wall_color
                {
                    if (x, y + 1) == self.end_point {
                        println!("Congrats, you've finished the maze!");
                        self.position = (x, y + 1);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                        self.game_over = true;
                    } else {
                        self.position = (x, y + 1);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                    }
                }
            }
            Direction::West => {
                if buffer.get(x as isize - 1, y as isize) != None
                    && buffer[(x - 1, y)] != self.maze_config.wall_color
                {
                    if (x - 1, y) == self.end_point {
                        println!("Congrats, you've finished the maze!");
                        self.position = (x - 1, y);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                        self.game_over = true;
                    } else {
                        self.position = (x - 1, y);
                        self.direction = Direction::Still;
                        self.previous_spot = (x, y);
                    }
                }
            }
            Direction::Still => {
                self.position = self.position.clone();
                self.previous_spot = self.previous_spot;
            }
        }
    }
}

pub fn display(player: &Player, buffer: &mut WindowBuffer) {
    buffer[player.previous_spot] = player.maze_config.path_color;
    buffer[player.position] = player.player_color;
    buffer[player.end_point] = player.finish_color;
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn test_generate_maze() {
        let mut buffer: WindowBuffer = WindowBuffer::new(6, 6);
        let mut rng = StdRng::seed_from_u64(38);
        MazeConfig::default().generate(&mut buffer, &mut rng);

        insta::assert_snapshot!(buffer, @r###"
        #.....
        #.#.##
        #.#...
        #####.
        #.....
        ####.#
        "###);
    }

    #[test]
    fn generate_start_and_end() {
        let mut buffer: WindowBuffer = WindowBuffer::new(3, 3);
        let mut rng = StdRng::seed_from_u64(38);
        let mut player = Player::new(
            (0, 0),
            (0, 0),
            Direction::Still,
            (0, 0),
            MazeConfig::default(),
            false,
        );
        start_end_generator(&mut buffer, &mut rng, &mut player);

        insta::assert_snapshot!(buffer, @r###"
        #..
        ...
        ..#
        "###);
    }

    #[test]
    fn direction_check() {
        let mut buffer: WindowBuffer = WindowBuffer::new(5, 5);
        let mut rng = StdRng::seed_from_u64(38);
        let mut player = Player::new(
            (0, 0),
            (0, 0),
            Direction::Still,
            (0, 0),
            MazeConfig::default(),
            false,
        );
        start_end_generator(&mut buffer, &mut rng, &mut player);

        insta::assert_snapshot!(buffer, @r###"
        .....
        #....
        .....
        ....#
        .....
        "###);

        player.direction = Direction::North;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        #....
        .....
        .....
        ....#
        .....
        "###);

        player.direction = Direction::North;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        #....
        .....
        .....
        ....#
        .....
        "###);

        player.direction = Direction::East;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        .#...
        .....
        .....
        ....#
        .....
        "###);

        player.direction = Direction::East;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ..#..
        .....
        .....
        ....#
        .....
        "###);

        player.direction = Direction::South;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        .....
        ..#..
        .....
        ....#
        .....
        "###);

        player.direction = Direction::South;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        .....
        .....
        ..#..
        ....#
        .....
        "###);

        player.direction = Direction::West;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        .....
        .....
        .#...
        ....#
        .....
        "###);

        player.direction = Direction::West;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        .....
        .....
        #....
        ....#
        .....
        "###);
    }
}
