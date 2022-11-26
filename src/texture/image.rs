use std::{fs::File, io::Read};

use crate::{model::vec3::Vec3, util::rtweekend::clamp};

use super::texture::Texture;

pub struct ImageTexture {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

const BYTES_PER_PIXEL: i32 = 3;

impl ImageTexture {
    pub fn new(filename: String) -> Self {
        let mut components_per_pixel = BYTES_PER_PIXEL;

        let mut f = File::open(filename).expect("file not found");
        let mut contents = vec![];
        f.read_to_end(&mut contents);

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut comp: i32 = 0;
        let img: *mut u8;

        unsafe {
            img = stb_image_rust::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut x,
                &mut y,
                &mut comp,
                components_per_pixel,
            );
        }

        let width = x;
        let height = y;
        let bytes_per_scanline = BYTES_PER_PIXEL * width;
        let data = unsafe {
            std::slice::from_raw_parts(img, (x * y * comp * components_per_pixel) as usize).to_vec()
        };

        unsafe {
            stb_image_rust::c_runtime::free(img);
        }

        Self {
            width,
            height,
            bytes_per_scanline,
            data,
        }
    }
}

impl Default for ImageTexture {
    fn default() -> Self {
        Self {
            data: Default::default(),
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &crate::model::vec3::Vec3) -> crate::model::vec3::Vec3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0, 1] x [1, 0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pos = (j * self.bytes_per_scanline + i * BYTES_PER_PIXEL) as usize;

        return Vec3::new(
            color_scale * self.data[pos] as f64,
            color_scale * self.data[pos + 1] as f64,
            color_scale * self.data[pos + 2] as f64,
        );
    }
}
