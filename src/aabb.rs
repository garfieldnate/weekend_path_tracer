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
        for a in 0..3 {
            let inv_direction = 1. / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_direction;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_direction;
            if inv_direction < 0. {
                mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
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
