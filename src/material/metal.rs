use crate::model::{hit::HitRecord, ray::Ray, vec3::Vec3};

use super::material::Material;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, f: f64) -> Self {
        Self {
            albedo: albedo.clone(),
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(
            &rec.p,
            &(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
            r_in.time(),
        );
        *attenuation = self.albedo.clone();
        scattered.dir().dot(&rec.normal) > 0.0
    }
}
