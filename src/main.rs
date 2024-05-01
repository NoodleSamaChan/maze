use core::time::Duration;
use graphic::{minifb::Minifb, Graphic, Key};
use maze::{display, start_end_generator, MazeConfig, Player};
use std::time::Instant;
use window_rs::WindowBuffer;

fn main() {
    let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);

    let mut rng = rand::thread_rng();
    let config = MazeConfig::default();
    config.generate(&mut buffer, &mut rng);
    let mut player = Player::new(
        (0, 0),
        (0, 0),
        maze::Direction::Still,
        (0, 0),
        config.clone(),
        false,
    );
    let start_point = start_end_generator(&mut buffer, &mut rng, &mut player);

    let mut window = Minifb::new("Maze - ESC to exit", buffer.width(), buffer.height());

    let mut update_time_wait = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) && player.game_over == false {
        let elapsed_time = Duration::from_millis(10 as u64);
        let _ = player.handle_user_input(&window, &start_point);

        if update_time_wait.elapsed() >= elapsed_time {
            display(&mut player, &mut buffer);
            player.direction(&buffer);
            update_time_wait = Instant::now();
        }

        window
            .update_with_buffer(&buffer)
    }
}
