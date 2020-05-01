use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn with_object(object: Box<dyn Hittable>) -> Self {
        let mut _self = Self::new();
        _self.add(object);
        _self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit_so_far: Option<HitRecord> = None;
        let mut closest_distance_so_far = t_max;

        for o in &self.objects {
            match o.hit(r, t_min, closest_distance_so_far) {
                Some(hit) => {
                    closest_hit_so_far = Some(hit);
                    closest_distance_so_far = hit.t;
                }
                None => {}
            }
        }

        closest_hit_so_far
    }
}
