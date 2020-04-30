
use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self { Self { origin, direction } }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}



// class ray {
//     public:
//         ray() {}
//         ray(const vec3& origin, const vec3& direction)
//             : orig(origin), dir(direction)
//         {}

//         vec3 origin() const    { return orig; }
//         vec3 direction() const { return dir; }

//         vec3 at(double t) const {
//             return orig + t*dir;
//         }

//     public:
//         vec3 orig;
//         vec3 dir;
// };

// #endif
