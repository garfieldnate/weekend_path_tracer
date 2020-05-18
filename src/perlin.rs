use crate::vec3::Vec3;
use rand::{prelude::SliceRandom, thread_rng};
use std::fmt::Debug;

#[derive(Clone, Copy)]
// TODO: really wish we could use a constant instead of a magic 256 everywhere
pub struct Perlin {
    random_vectors: [Vec3; 256],
    perm_x: [u8; 256],
    perm_y: [u8; 256],
    perm_z: [u8; 256],
}

impl Debug for Perlin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Perlin")
    }
}

struct ArrayPrinter {
    data: [f64; 256],
}

impl Debug for ArrayPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data[..].fmt(f)
    }
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vectors = [Vec3::default(); 256];
        for value in random_vectors.iter_mut() {
            *value = Vec3::random_in_range(-1., 1.).norm();
        }

        // eprintln!(
        //     "{:?}",
        //     ArrayPrinter {
        //         data: random_vectors
        //     }
        // );

        let perm_x = generate_permutation();
        // eprintln!("perm_x: {:?}", ArrayPrinter { data: perm_x });
        let perm_y = generate_permutation();
        // eprintln!("perm_y: {:?}", ArrayPrinter { data: perm_y });
        let perm_z = generate_permutation();
        // eprintln!("perm_z: {:?}", ArrayPrinter { data: perm_z });

        Self {
            random_vectors,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();

        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let component_i = self.perm_x[(i + di as f64) as usize & 255] as usize;
                    let component_j = self.perm_y[(j + dj as f64) as usize & 255] as usize;
                    let component_k = self.perm_z[(k + dk as f64) as usize & 255] as usize;
                    c[di][dj][dk] = self.random_vectors[component_i ^ component_j ^ component_k];
                }
            }
        }

        // return trilinear_interpolation(c, u, v, w);
        return perlin_interpolation(c, u, v, w);
    }

    pub fn turbulence(&self, p: Vec3, depth: u8) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        return accum.abs();
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

fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_float = i as f64;
                let j_float = j as f64;
                let k_float = k as f64;
                let weight_v = Vec3::new(u - i_float, v - j_float, w - k_float);
                accum += (i_float * uu + (1. - i_float) * (1. - uu))
                    * (j_float * vv + (1. - j_float) * (1. - vv))
                    * (k_float * ww + (1. - k_float) * (1. - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }
    return accum;
}
