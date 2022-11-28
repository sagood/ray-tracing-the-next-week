use std::sync::Arc;

use crate::material::material::Material;

use super::{aabb::Aabb, hit::Hittable, vec3::Vec3};

pub struct XyRect {
    mp: Arc<dyn Material + Sync + Send>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp: mat,
        }
    }
}

impl Hittable for XyRect {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().z()) / r.dir().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.dir().x();
        let y = r.origin().y() + t * r.dir().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.mp.clone();
        rec.p = r.at(t);
        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        *output_box = Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}
