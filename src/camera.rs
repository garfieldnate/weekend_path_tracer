use crate::{
    ray::Ray,
    utils::{random_in_range, random_in_unit_disk},
    vec3::Vec3,
};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    // vertical_field_of_view is top to bottom in degrees; aspect = width to height ratio
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vertical_field_of_view: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let origin = look_from;
        let lens_radius = aperture / 2.;

        let theta = vertical_field_of_view.to_radians();
        let half_height = (theta / 2.).tan();
        let half_height = aspect * half_height;
        let half_width = aspect * half_height;
        let w = (look_from - look_at).norm();
        let u = view_up.cross(w).norm();
        let v = w.cross(u);

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2. * half_width * focus_dist * u;
        let vertical = 2. * half_height * focus_dist * v;

        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_in_range(self.time0, self.time1),
        );
    }
}
