use maze::{generate_maze, start_end_generator};
use minifb::{Key, Window, WindowOptions};
use window_rs::WindowBuffer;

fn main() {
    let mut buffer: WindowBuffer = WindowBuffer::new(30, 30);

    let mut rng = rand::thread_rng();
    generate_maze(&mut buffer, &mut rng);
    start_end_generator(&mut buffer, &mut rng);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.buffer(), buffer.width(), buffer.height())
            .unwrap();
    }
}
