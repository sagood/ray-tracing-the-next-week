use crate::model::vec3::Vec3;

use Vec3 as Point3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Vec3;
}
