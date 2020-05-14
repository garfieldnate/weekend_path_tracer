use crate::{perlin::Perlin, vec3::Vec3};
use std::{fmt::Debug, sync::Arc};

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
}
impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            perlin: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1., 1., 1.) * self.perlin.noise(p)
    }
}
