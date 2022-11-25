use crate::model::vec3::Vec3;

use super::{perlin::Perlin, texture::Texture};

pub struct NoiseTexture {
    pub noise: Perlin,
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &crate::model::vec3::Vec3) -> crate::model::vec3::Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
