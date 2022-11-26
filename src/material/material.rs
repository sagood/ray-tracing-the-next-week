use crate::model::{hit::HitRecord, ray::Ray, vec3::Vec3};

use Vec3 as Point3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
