use weekend_path_tracer::{vec3::Vec3, canvas::Canvas};
const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn get_hello_world_image_data() -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color = Vec3::new(i as f64/ IMAGE_WIDTH as f64, j as f64/IMAGE_HEIGHT as f64, 0.2);
            let (r,g,b) = color.to_rgb();
            let rgb = from_u8_rgb(r,g,b);
            buffer.push(rgb);
        }
    }
    buffer
}

fn main() {
    let buffer = get_hello_world_image_data();
    let canvas = Canvas {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
        data: buffer,
    };
    canvas.display_image();
}
