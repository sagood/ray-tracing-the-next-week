use crate::model::vec3::Vec3;

use super::texture::Texture;

pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn new(color: &Vec3) -> Self {
        Self {
            color_value: color.clone(),
        }
    }

    pub fn new_with_values(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor::new(&Vec3::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color_value
    }
}
