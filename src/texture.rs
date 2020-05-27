use crate::{perlin::Perlin, vec3::Vec3};
use std::{fmt::Debug, sync::Arc, path::Path};
use image;

pub trait Texture: Sync + Send + Debug {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct SolidColor {
    color: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color
    }
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color: Vec3::new(r, g, b),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoiseTexture {
    pub perlin: Perlin,
    pub scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        // Perlin output is negative, and gamma function takes sqrt() later, so make the result positive here
        // Vec3::new(1., 1., 1.) * 0.5 * (1. + self.perlin.noise(self.scale * p))
        // Vec3::new(1., 1., 1.) * 0.5 * (1. + self.perlin.turbulence(p, 7))
        Vec3::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.z() + 10. * self.perlin.turbulence(p, 7)).sin())
        // (1 + sin(scale*p.z() + 10*noise.turb(p)));
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image_result = image::open(path).unwrap().to_rgb();// TODO: that's not safe...
        // auto components_per_pixel = bytes_per_pixel;

        // data = stbi_load(
        //     filename, &width, &height, &components_per_pixel, components_per_pixel);

        // if (!data) {
        //     std::cerr << "ERROR: Could not load texture image file '" << filename << "'.\n";
        //     width = height = 0;
        // }

        // bytes_per_scanline = bytes_per_pixel * width;
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
            // If we have no texture data, then return solid cyan as a debugging aid.
            if self.data == nullptr {
                return color(0,1,1);
            }

            // Clamp input texture coordinates to [0,1] x [1,0]
            let u = u.max(0.).min(1.);
            v = 1. - v.max(0.).min(1.);  // Flip V to image coordinates

            let i = (u * width) as usize;
            let j = (v * height) as usize;

            // Clamp integer mapping, since actual coordinates should be less than 1.0
            let i = if (i >= width)  {width-1} else {i};
            let j = if (j >= height) {j = height-1} else {j};

            let color_scale = 1. / 255.;
            let pixel = self.data[j][i];

            return Vec3::new(color_scale*pixel[0], color_scale*pixel[1], color_scale*pixel[2]);
    }
}
