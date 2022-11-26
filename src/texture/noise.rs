use crate::model::vec3::Vec3;

use super::{perlin::Perlin, texture::Texture};

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            scale: sc,
            noise: Perlin::new(),
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &crate::model::vec3::Vec3) -> crate::model::vec3::Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&(self.scale * p), 7)).sin())
    }
}
