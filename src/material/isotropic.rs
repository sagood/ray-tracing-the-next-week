use std::sync::Arc;

use crate::{
    model::{ray::Ray, vec3::Vec3},
    texture::{solid_color::SolidColor, texture::Texture},
};

use super::material::Material;

pub struct Isotropic {
    pub albedo: Arc<dyn Texture + Sync + Send>,
}

impl Isotropic {
    pub fn new(a: Arc<dyn Texture + Sync + Send>) -> Self {
        Self { albedo: a }
    }

    pub fn new_with_color(c: Vec3) -> Isotropic {
        let albedo = Arc::new(SolidColor::new(&c));

        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &crate::model::ray::Ray,
        rec: &crate::model::hit::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::model::ray::Ray,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        return true;
    }
}
