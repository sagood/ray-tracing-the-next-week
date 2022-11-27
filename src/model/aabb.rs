use super::{ray::Ray, vec3::Vec3};

use Vec3 as Point3;

#[derive(Clone)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;

        for a in 0..3 {
            let t0 = ((self.minimum[a] - r.origin()[a]) / (r.dir())[a])
                .min((self.maximum[a] - r.origin()[a]) / (r.dir())[a]);
            let t1 = ((self.minimum[a] - r.origin()[a]) / (r.dir())[a])
                .max((self.maximum[a] - r.origin()[a]) / (r.dir())[a]);
            tmin = t0.max(tmin);
            tmax = t1.min(tmax);
            if tmax < tmin {
                return false;
            }
        }

        return true;
    }

    pub fn surrounding_box(&self, box1: &Aabb) -> Aabb {
        let small = Vec3::new(
            self.minimum.x().min(box1.minimum.x()),
            self.minimum.y().min(box1.minimum.y()),
            self.minimum.z().min(box1.minimum.z()),
        );
        let big = Vec3::new(
            self.maximum.x().max(box1.maximum.x()),
            self.maximum.y().max(box1.maximum.y()),
            self.maximum.z().max(box1.maximum.z()),
        );

        Aabb::new(small, big)
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            minimum: Default::default(),
            maximum: Default::default(),
        }
    }
}
