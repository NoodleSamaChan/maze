use core::time::Duration;
use maze::{display, start_end_generator, MazeConfig, Player};
use minifb::{Key, Window, WindowOptions};
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
            .update_with_buffer(&buffer.buffer(), buffer.width(), buffer.height())
            .unwrap();
    }
}
