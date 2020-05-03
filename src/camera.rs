use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // vertical_field_of_view is top to bottom in degrees; aspect = width to height ratio
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vertical_field_of_view: f64,
        aspect: f64,
    ) -> Self {
        let origin = look_from;

        let theta = vertical_field_of_view.to_radians();
        let half_height = (theta / 2.).tan();
        let half_height = aspect * half_height;
        let half_width = aspect * half_height;
        let w = (look_from - look_at).norm();
        let u = view_up.cross(w).norm();
        let v = w.cross(u);

        let lower_left_corner = origin - half_width * u - half_height * v - w;

        let horizontal = 2. * half_width * u;
        let vertical = 2. * half_height * v;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
