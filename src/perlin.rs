use crate::{utils::random_in_01, vec3::Vec3};
use rand::{prelude::SliceRandom, thread_rng};

// TODO: really wish we could use a constant instead of a magic 256 everywhere
pub struct Perlin {
    random_floats: [f64; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_floats = [0.; 256];
        for value in random_floats.iter_mut() {
            *value = random_in_01();
        }

        Self {
            random_floats,
            perm_x: generate_permutation(),
            perm_y: generate_permutation(),
            perm_z: generate_permutation(),
        }
    }
    pub fn noise(&self, p: Vec3) -> f64 {
        // let u = p.x() - p.x().floor();
        // let v = p.y() - p.y().floor();
        // let w = p.z() - p.z().floor();

        let i = (4. * p.x()) as usize & 255;
        let j = (4. * p.y()) as usize & 255;
        let k = (4. * p.z()) as usize & 255;

        return self.random_floats[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize];
    }
}

fn generate_permutation() -> [u8; 256] {
    let mut p = [0u8; 256];
    // TODO: Rust should allow exclusive range with 256 below!
    for value in 0..=255 {
        p[value as usize] = value;
    }

    p.shuffle(&mut thread_rng());

    p
}
