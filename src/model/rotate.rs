use std::sync::Arc;

use crate::util::rtweekend::{degrees_to_radians, INFINITY};

use super::{aabb::Aabb, hit::Hittable, ray::Ray, vec3::Vec3};

use Vec3 as Point3;

pub struct RotateY {
    pub hittable: Arc<dyn Hittable + Sync + Send>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_box: bool,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable + Sync + Send>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::default();
        let has_box = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x() + (1.0 - i as f64) * bbox.minimum.x();
                    let y = j as f64 * bbox.maximum.y() + (1.0 - j as f64) * bbox.minimum.y();
                    let z = k as f64 * bbox.maximum.z() + (1.0 - k as f64) * bbox.minimum.z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::new(min, max);

        Self {
            hittable: p,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let mut origin = r.origin().clone();
        let mut direction = r.dir().clone();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.dir()[0] - self.sin_theta * r.dir()[2];
        direction[2] = self.sin_theta * r.dir()[0] + self.cos_theta * r.dir()[2];

        let rotated_r = Ray::new(&origin, &direction, r.time());

        if !self.hittable.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox.clone();
        return self.has_box;
    }
}
