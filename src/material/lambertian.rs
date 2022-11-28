use std::sync::Arc;

use crate::{
    model::{hit::HitRecord, ray::Ray, vec3::Vec3},
    texture::{solid_color::SolidColor, texture::Texture},
};

use super::material::Material;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture + Sync + Send>,
}

impl Lambertian {
    pub fn new(a: &Vec3) -> Lambertian {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }

    pub fn new_with_texture(a: Arc<dyn Texture + Sync + Send>) -> Lambertian {
        Lambertian { albedo: a.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        return true;
    }
}
