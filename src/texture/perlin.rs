use crate::{
    model::vec3::Vec3,
    util::rtweekend::{random_double, random_int},
};

use Vec3 as Point3;
const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = Vec::new();
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::random_by_range(-1.0, 1.0).unit_vector());
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let mut u = p.x() - (p.x()).floor();
        let mut v = p.y() - (p.y()).floor();
        let mut w = p.z() - (p.z()).floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = vec![vec![vec![Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }

        Perlin::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv as f64))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww as f64))
                        * c[i][j][k].dot(&weight_v);
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
            ranvec: Default::default(),
            perm_x: Default::default(),
            perm_y: Default::default(),
            perm_z: Default::default(),
        }
    }
}
