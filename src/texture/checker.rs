use std::sync::Arc;

use crate::model::vec3::Vec3;

use super::{solid_color::SolidColor, texture::Texture};

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture + Sync + Send>,
    pub even: Arc<dyn Texture + Sync + Send>,
}

impl CheckerTexture {
    pub fn new(
        _even: &Arc<dyn Texture + Sync + Send>,
        _odd: &Arc<dyn Texture + Sync + Send>,
    ) -> Self {
        Self {
            odd: _odd.clone(),
            even: _even.clone(),
        }
    }

    pub fn new_with_color(c1: &Vec3, c2: &Vec3) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c2)),
            even: Arc::new(SolidColor::new(c1)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, &p);
        } else {
            return self.even.value(u, v, &p);
        }
    }
}
