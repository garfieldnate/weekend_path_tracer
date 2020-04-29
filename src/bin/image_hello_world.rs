use minifb::{Key, Window, WindowOptions};
const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn display_image(buffer: Vec<u32>) {
    // taken verbatim from minifb readme;
    // TODO: can we just display an image without repeatedly updating the frame buffer?
    let mut window = Window::new(
        "Hello, world! - ESC to exit",
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
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
            .update_with_buffer(&buffer, IMAGE_WIDTH, IMAGE_HEIGHT)
            .unwrap();
    }
}

fn get_image_data() -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_HEIGHT as f64;
            let b = 0.2f64;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            let rgb = from_u8_rgb(ir, ig, ib);
            buffer.push(rgb);
        }
    }
    buffer
}

fn main() {
    let buffer = get_image_data();
    display_image(buffer);
}

// rayon = "1.1"

//     pub fn write_data_parallel<F>(&mut self, processor: F)
//     where
//         F: Fn(usize, usize, &mut Vector3d) + Sync,
//     {
//         self.data
//             .par_iter_mut()
//             .enumerate() // generate an index for each column we're iterating
//             .for_each(|(col_index, row)| {
//                 for (row_index, pixel) in row.iter_mut().enumerate() {
//                     processor(row_index, col_index, pixel);
//                 }
//             });
//     }
