use std::sync::Arc;

use Vec3 as Point3;

use crate::material::material::Material;

use super::{
    aabb::Aabb,
    hit::{Hittable, HittableList},
    vec3::Vec3,
    xy_rect::XyRect,
    xz_rect::XzRect,
    yz_rect::YzRect,
};

pub struct Box {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl Box {
    pub fn new(p0: &Point3, p1: &Point3, mat: Arc<dyn Material + Sync + Send>) -> Self {
        let box_min = p0.clone();
        let box_max = p1.clone();

        let mut sides = HittableList::new();
        sides.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        sides.add(Arc::new(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )));

        sides.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )));

        sides.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Box {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::Aabb) -> bool {
        *output_box = Aabb::new(self.box_min, self.box_max);
        return true;
    }
}
