use crate::util::rtweekend::clamp;

use super::vec3::Vec3;

pub trait Color {
    fn as_color_repr(&self, samples_per_pixel: usize) -> String;
}

impl Color for Vec3 {
    fn as_color_repr(&self, samples_per_pixel: usize) -> String {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide the color by the number of samples and gamma-correct for gamma=2.0
        let scale = 1.0 / samples_per_pixel as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        format!(
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)) as i32,
            (256.0 * clamp(g, 0.0, 0.999)) as i32,
            (256.0 * clamp(b, 0.0, 0.999)) as i32,
        )
    }
}
