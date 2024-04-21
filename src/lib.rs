use rand::prelude::SliceRandom;
use rand::Rng;
use window_rs::WindowBuffer;

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
