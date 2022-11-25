use crate::{
    model::vec3::Vec3,
    util::rtweekend::{random_double, random_int},
};

use Vec3 as Point3;
const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = Vec::new();
        for _ in 0..POINT_COUNT {
            ranfloat.push(random_double())
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - (p.x()).floor();
        let v = p.y() - (p.y()).floor();
        let w = p.z() - (p.z()).floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = vec![vec![vec![0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v as f64))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w as f64))
                        * c[i][j][k];
                }
            }
        }

        accum
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Perlin::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (1..n).rev() {
            let target = random_int(0, i as i32);
            let tmp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            ranfloat: Default::default(),
            perm_x: Default::default(),
            perm_y: Default::default(),
            perm_z: Default::default(),
        }
    }
}
