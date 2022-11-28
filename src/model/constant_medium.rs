use std::sync::Arc;

use crate::{
    material::{isotropic::Isotropic, material::Material},
    texture::texture::Texture,
    util::rtweekend::{random_double, INFINITY},
};

use super::{
    hit::{HitRecord, Hittable},
    vec3::Vec3,
};

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable + Sync + Send>,
    pub phase_function: Arc<dyn Material + Sync + Send>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_with_texture(
        b: Arc<dyn Hittable + Sync + Send>,
        d: f64,
        a: Arc<dyn Texture + Sync + Send>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }

    pub fn new(b: Arc<dyn Hittable + Sync + Send>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_with_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        let enable_debugging = false;
        let debugging = enable_debugging && random_double() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2: HitRecord = HitRecord::default();

        if !self.boundary.hit(&r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(&r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().log10();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = self.phase_function.clone();

        return true;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
