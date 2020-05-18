use crate::{utils::random_in_01, vec3::Vec3};
use rand::{prelude::SliceRandom, thread_rng};
use std::fmt::Debug;

#[derive(Clone, Copy)]
// TODO: really wish we could use a constant instead of a magic 256 everywhere
pub struct Perlin {
    random_floats: [f64; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256],
}

impl Debug for Perlin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Perlin")
    }
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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = (4. * p.x()) as usize & 255;
        let j = (4. * p.y()) as usize & 255;
        let k = (4. * p.z()) as usize & 255;

        let mut c = [[[0.; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let component_i = self.perm_x[(i + di) & 255] as usize;
                    let component_j = self.perm_y[(j + dj) & 255] as usize;
                    let component_k = self.perm_z[(k + dk) & 255] as usize;
                    c[di][dj][dk] = self.random_floats[component_i ^ component_j ^ component_k];
                }
            }
        }

        return trilinear_interpolation(c, u, v, w);
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

fn trilinear_interpolation(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_float = i as f64;
                let j_float = j as f64;
                let k_float = k as f64;
                acc += (i_float * u + (1. - i_float) * (1. - u))
                    * (j_float * v + (1. - j_float) * (1. - v))
                    * (k_float * w + (1. - k_float) * (1. - w))
                    * c[i][j][k];
            }
        }
    }

    return acc;
}
