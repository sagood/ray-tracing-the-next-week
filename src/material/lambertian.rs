use crate::model::{hit::HitRecord, ray::Ray, vec3::Vec3};

use super::material::Material;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Lambertian {
        Self {
            albedo: albedo.clone(),
        }
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

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
    }
}
