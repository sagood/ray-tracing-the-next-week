use std::{
    cmp::Ordering,
    io::{self, Write},
    sync::Arc,
};

use crate::util::rtweekend::random_int;

use super::{
    aabb::Aabb,
    hit::{Hittable, HittableList},
    vec3::Vec3,
};

pub struct BvhNode {
    pub left: Arc<dyn Hittable + Sync + Send>,
    pub right: Arc<dyn Hittable + Sync + Send>,
    pub bounding_box: Aabb,
}

impl BvhNode {
    pub fn new_with_list(list: &HittableList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::new(&list.objects, 0, list.objects.len(), time0, time1)
    }
    pub fn new(
        src_objects: &Vec<Arc<dyn Hittable + Sync + Send>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects = src_objects.clone();

        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = end - start;
        let mut left;
        let mut right;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b));

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new(&objects, start, mid, time0, time1));
            right = Arc::new(BvhNode::new(&objects, mid, end, time0, time1));
        }

        let mut box_left = Aabb::new(Vec3::default(), Vec3::default());
        let mut box_right = Aabb::new(Vec3::default(), Vec3::default());

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            eprint!("No bounding box in bvh_node constructor.\n");
            io::stderr().flush().unwrap();
        }

        let bounding_box = box_left.surrounding_box(&box_right);
        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &super::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hit::HitRecord,
    ) -> bool {
        if self.bounding_box.hit(r, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, t_max, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bounding_box.clone();
        return true;
    }
}

fn box_compare(
    a: &Arc<dyn Hittable + Sync + Send>,
    b: &Arc<dyn Hittable + Sync + Send>,
    axis: i32,
) -> Ordering {
    let mut box_a = Aabb::new(Vec3::default(), Vec3::default());
    let mut box_b = Aabb::new(Vec3::default(), Vec3::default());

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprint!("No bounding box in bvh_node constructor.\n");
        io::stderr().flush().unwrap();
    }

    if box_a.minimum[axis] < box_b.minimum[axis] {
        Ordering::Less
    } else if box_a.minimum[axis] > box_b.minimum[axis] {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn box_x_compare(
    a: &Arc<dyn Hittable + Sync + Send>,
    b: &Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(
    a: &Arc<dyn Hittable + Sync + Send>,
    b: &Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(
    a: &Arc<dyn Hittable + Sync + Send>,
    b: &Arc<dyn Hittable + Sync + Send>,
) -> Ordering {
    box_compare(a, b, 2)
}
