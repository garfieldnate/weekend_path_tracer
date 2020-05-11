use crate::vec3::Vec3;
use std::fmt::Debug;

pub trait Texture: Sync + Send + Debug {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct SolidColor {
    color: Vec3,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
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
