use std::sync::Arc;

use super::{aabb::Aabb, hit::Hittable, ray::Ray, vec3::Vec3};

pub struct Translate {
    hittable: Arc<dyn Hittable + Sync + Send>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, displacement: &Vec3) -> Self {
        Self {
            hittable: p,
            offset: displacement.clone(),
        }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let moved_r = Ray::new(&(r.origin() - self.offset), r.dir(), r.time());
        if !self.hittable.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &rec.normal.clone());

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        if !self.hittable.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = Aabb::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );
        return true;
    }
}
