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

pub fn start_end_generator (buffer: &mut WindowBuffer, rng: &mut impl Rng) -> (usize, usize) {
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

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Still,
}

pub struct Player {
    position: (usize, usize),
    direction: Direction,
}
impl Player {
    pub fn new(position: (usize, usize), direction: Direction,) -> Self {
        Self{position,
        direction,}
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
        .#####
        .#.#..
        .#.###
        .....#
        .#####
        ......
        "###);
    }
}
