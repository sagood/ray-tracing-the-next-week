use super::vec3::Vec3;
use Vec3 as Point3;

pub struct Ray {
    origin: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: &Point3, dir: &Vec3, time: f64) -> Self {
        Self {
            origin: origin.clone(),
            dir: dir.clone(),
            tm: time,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}
