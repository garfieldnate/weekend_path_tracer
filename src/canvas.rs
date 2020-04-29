use minifb::{Key, Window, WindowOptions};
pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pub data: Vec<u32>,
}

impl Canvas {
    pub fn display_image(&self) {
        // taken verbatim from minifb readme;
        // TODO: can we just display an image without repeatedly updating the frame buffer?
        let mut window = Window::new(
            "Hello, world! - ESC to exit",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&self.data, self.width, self.height)
                .unwrap();
        }
    }
}
