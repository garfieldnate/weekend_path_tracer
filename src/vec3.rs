use crate::utils::{clamp, random_in_01, random_in_range};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vec3 {
    // TODO: this really shouldn't be public
    pub data: [f64; 3],
}

const COLOR_MAX: f64 = 256.;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }
    pub fn random() -> Self {
        Self {
            data: [random_in_01(), random_in_01(), random_in_01()],
        }
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self {
            data: [
                random_in_range(min, max),
                random_in_range(min, max),
                random_in_range(min, max),
            ],
        }
    }
    pub fn x(&self) -> f64 {
        self.data[0]
    }
    pub fn y(&self) -> f64 {
        self.data[1]
    }
    pub fn z(&self) -> f64 {
        self.data[2]
    }
    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }
    pub fn magnitude(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn norm(&self) -> Vec3 {
        let magnitude = self.magnitude();
        Self::new(
            self.x() / magnitude,
            self.y() / magnitude,
            self.z() / magnitude,
        )
    }
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
    // assumes that x,y,z are in [0,1], outputs (r,g,b)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        // apply gamma correction (gamma 2, or raised to the 1/2 power)
        let r = self.x().sqrt();
        let g = self.y().sqrt();
        let b = self.z().sqrt();
        (
            (COLOR_MAX * clamp(r, 0., 0.999)) as u8,
            (COLOR_MAX * clamp(g, 0., 0.999)) as u8,
            (COLOR_MAX * clamp(b, 0., 0.999)) as u8,
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "vector({}, {}, {})", self.x(), self.y(), self.z())?;
        Ok(())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

// Hadamard product is used for mixing two colors together
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Self::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, tuple: Vec3) -> Vec3 {
        tuple * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self::new(self.x() * scalar, self.y() * scalar, self.z() * scalar);
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        let inv_scalar = 1. / scalar;
        self * inv_scalar
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        let inv_scalar = 1. / scalar;
        *self = Self::new(
            self.x() * inv_scalar,
            self.y() * inv_scalar,
            self.z() * inv_scalar,
        );
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
