use std::sync::Arc;

use crate::material::material::Material;

use super::{aabb::Aabb, hit::Hittable, vec3::Vec3};

pub struct XzRect {
    mp: Arc<dyn Material + Sync + Send>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().y()) / r.dir().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.dir().x();
        let z = r.origin().z() + t * r.dir().z();
        if z < self.z0 || z > self.z1 || x < self.x0 || x > self.x1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        // The bounding box must have non-zero width in each dimension, so pad the Y dimension a small amount
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        return true;
    }
}
