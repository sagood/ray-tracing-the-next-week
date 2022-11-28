use std::sync::Arc;

use crate::{
    model::vec3::Vec3,
    texture::{solid_color::SolidColor, texture::Texture},
};

use super::material::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture + Sync + Send>,
}

impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { emit: a }
    }

    pub fn new_with_color(c: Vec3) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(&c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &crate::model::ray::Ray,
        rec: &crate::model::hit::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::model::ray::Ray,
    ) -> bool {
        return false;
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
