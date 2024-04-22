use rand::prelude::SliceRandom;
use rand::Rng;
use window_rs::WindowBuffer;
use minifb::{Window, Key, KeyRepeat};

const VISITED_COLOR: u32 = 0x0000ff; 
const FREE_SPACE: u32 = 0;

pub fn generate_maze(buffer: &mut WindowBuffer, rng: &mut impl Rng) {
    buffer.reset();

    let x = rng.gen_range(0..buffer.width());
    let y = rng.gen_range(0..buffer.height());

    let mut stack = Vec::new();
    stack.push((x, y));
    buffer[(x, y)] = VISITED_COLOR;

    while let Some(cell) = stack.pop() {
        let (x, y) = (cell.0 as isize, cell.1 as isize);

        let mut to_explore = Vec::new();

        if let Some(cell) = buffer.get(x - 2, y) {
            if cell == FREE_SPACE {
                let (x, y) = (x as usize, y as usize);
                to_explore.push((x as usize - 2, y as usize));
            }
        }
        if let Some(cell) = buffer.get(x + 2, y) {
            if cell == FREE_SPACE {
                let (x, y) = (x as usize, y as usize);
                to_explore.push((x as usize + 2, y as usize));
            }
        }
        if let Some(cell) = buffer.get(x, y - 2) {
            if cell == FREE_SPACE {
                let (x, y) = (x as usize, y as usize);
                to_explore.push((x as usize, y as usize - 2));
            }
        }
        if let Some(cell) = buffer.get(x, y + 2) {
            if cell == FREE_SPACE {
                let (x, y) = (x as usize, y as usize);
                to_explore.push((x as usize, y as usize + 2));
            }
        }

        if let Some(cell2) = to_explore.choose(rng) {
            if to_explore.len() > 1 {
                stack.push(cell);
            }
            stack.push(*cell2);
            buffer[*cell2] = VISITED_COLOR;
            let wall = middle_point(cell, *cell2);
            buffer[wall] = VISITED_COLOR;
        }
    }
    for x in 0..buffer.width() {
        for y in 0..buffer.height() {
            if buffer[(x, y)] == FREE_SPACE {
                buffer[(x, y)] = VISITED_COLOR
            } else {
                buffer[(x, y)] = FREE_SPACE
            }
        }
    }
}

pub fn start_end_generator (buffer: &mut WindowBuffer, rng: &mut impl Rng, player: &mut Player) -> (usize, usize) {
    let mut start_point_ready = false;
    let mut end_point_ready = false;
    loop {
        let start_height = rng.gen_range(0..buffer.height());
        let end_height = rng.gen_range(0..buffer.height());
        let width_max = buffer.width() - 1;

        if start_point_ready == false && buffer[(1, start_height)] == 0 {
            buffer[(0, start_height)] = 0x00FF00;
            start_point_ready = true;
        }
        if end_point_ready == false && buffer[(&width_max - 1, end_height)] == 0 {
            buffer[(width_max, end_height)] = 0xFF00FF;
            end_point_ready = true;
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Player {
    position: (usize, usize),
    end_point: (usize, usize),
    direction: Direction,
    previous_spot: (usize, usize), 
    pub game_over : bool,
}
impl Player {
    pub fn new(position: (usize, usize), end_point: (usize, usize), direction: Direction, previous_spot: (usize, usize), game_over : bool) -> Self {
        Self{position,
            end_point,
            direction,
            previous_spot,
            game_over,}
    }

pub fn handle_user_input(
        &mut self,
        window: &Window,
        start_point: &(usize, usize)
    ) -> std::io::Result<()> {
        if window.is_key_pressed(Key::Q, KeyRepeat::No) {
            self.reset(*start_point);
        }

        if window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            self.direction = Direction::North;
        }

        if window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            self.direction = Direction::South;
        }

        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            self.direction = Direction::East;
        }

        if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
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
                if buffer.get(x as isize + 1, y as isize) != None && buffer[(x + 1, y)] != VISITED_COLOR {
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
                if buffer.get(x as isize, y as isize - 1) != None && buffer[(x, y - 1)] != VISITED_COLOR {
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
                if buffer.get(x as isize, y as isize + 1) != None && buffer[(x, y + 1)] != VISITED_COLOR {
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
                if buffer.get(x as isize - 1, y as isize) != None && buffer[(x - 1, y)] != VISITED_COLOR {
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

pub fn display(player: &Player, buffer: &mut WindowBuffer){
    buffer[player.previous_spot] = FREE_SPACE;
    buffer[player.position] = 0x00FF00;
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn test_generate_maze() {
        let mut buffer: WindowBuffer = WindowBuffer::new(6, 6);
        let mut rng = StdRng::seed_from_u64(38);
        generate_maze(&mut buffer, &mut rng);

        insta::assert_snapshot!(buffer, @r###"
        #.....
        #.#.##
        #.#...
        #####.
        #.....
        ######
        "###);
    }

    #[test]
    fn generate_start_and_end() {
        let mut buffer: WindowBuffer = WindowBuffer::new(10, 10);
        let mut rng = StdRng::seed_from_u64(38);
        let mut player = Player::new((0, 0), (0, 0), Direction::Still, (0, 0), false);
        start_end_generator(&mut buffer, &mut rng, &mut player);

        insta::assert_snapshot!(buffer, @r###"
        ..........
        ..........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);
    }

    #[test]
    fn direction_check() {
        let mut buffer: WindowBuffer = WindowBuffer::new(10, 10);
        let mut rng = StdRng::seed_from_u64(38);
        let mut player = Player::new((0, 0), (0, 0), Direction::Still, (0, 0), false);
        start_end_generator(&mut buffer, &mut rng, &mut player);

        insta::assert_snapshot!(buffer, @r###"
        ..........
        ..........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::North;
        player.direction(&buffer);
        display(&mut player, &mut buffer);


        insta::assert_snapshot!(buffer, @r###"
        ..........
        #.........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::North;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        #.........
        #.........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::East;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ##........
        #.........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::East;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ###.......
        #.........
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::South;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ###.......
        #.#.......
        #.........
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::South;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ###.......
        #.#.......
        #.#.......
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::West;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ###.......
        #.#.......
        ###.......
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);

        player.direction = Direction::West;
        player.direction(&buffer);
        display(&mut player, &mut buffer);

        insta::assert_snapshot!(buffer, @r###"
        ###.......
        #.#.......
        ###.......
        ..........
        ..........
        ..........
        ..........
        .........#
        ..........
        ..........
        "###);
    }
}
