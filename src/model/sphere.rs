use std::sync::Arc;

use crate::material::material::Material;

use super::{hit::Hittable, vec3::Vec3};

use Vec3 as Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Arc<dyn Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let oc = r.origin() - self.center;
        let a = r.dir().length_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = self.material.clone();

        return true;
    }
}
