use std::sync::Arc;

use crate::material::material::Material;

use super::{aabb::Aabb, hit::Hittable, vec3::Vec3};

pub struct YzRect {
    mp: Arc<dyn Material + Sync + Send>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().x()) / r.dir().x();
        if t < t_min || t > t_max {
            return false;
        }

        let y = r.origin().y() + t * r.dir().y();
        let z = r.origin().z() + t * r.dir().z();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        // The bounding box must have non-zero width in each dimension, so pad the X dimension a small amount
        *output_box = Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        return true;
    }
}
