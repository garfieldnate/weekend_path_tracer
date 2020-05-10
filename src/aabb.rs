use crate::{ray::Ray, vec3::Vec3};
use std::mem;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        // TODO: the implementation from the book might actually depend on the
        // structure of a vec3 being an array. Without the array, this is an
        // unrolled loop. Might have less simd or something.
        let inverse_direction = 1. / r.direction().x();
        let mut t_0 = (self.min().x() - r.origin().x()) * inverse_direction;
        let mut t_1 = (self.max().x() - r.origin().x()) * inverse_direction;
        if inverse_direction < 0. {
            mem::swap(&mut t_0, &mut t_1);
        }

        t_min = if t_0 > t_min { t_0 } else { t_min };
        t_max = if t_1 < t_max { t_1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let inverse_direction = 1. / r.direction().y();
        let mut t_0 = (self.min().y() - r.origin().y()) * inverse_direction;
        let mut t_1 = (self.max().y() - r.origin().y()) * inverse_direction;
        if inverse_direction < 0. {
            mem::swap(&mut t_0, &mut t_1);
        }

        t_min = if t_0 > t_min { t_0 } else { t_min };
        t_max = if t_1 < t_max { t_1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let inverse_direction = 1. / r.direction().z();
        let mut t_0 = (self.min().z() - r.origin().z()) * inverse_direction;
        let mut t_1 = (self.max().z() - r.origin().z()) * inverse_direction;
        if inverse_direction < 0. {
            mem::swap(&mut t_0, &mut t_1);
        }

        t_min = if t_0 > t_min { t_0 } else { t_min };
        t_max = if t_1 < t_max { t_1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        return true;
    }
    pub fn combine(&self, other: AABB) -> Self {
        let small = Vec3::new(
            self.min().x().min(other.min().x()),
            self.min().y().min(other.min().y()),
            self.min().z().min(other.min().z()),
        );

        let big = Vec3::new(
            self.max().x().max(other.max().x()),
            self.max().y().max(other.max().y()),
            self.max().z().max(other.max().z()),
        );

        return Self::new(small, big);
    }
}
