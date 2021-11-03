use image::{GrayImage, RgbImage};
use image::io::Reader as ImageReader;
use cgmath::Vector3;
use crate::material::Material;

pub enum MaterialValue<T, U> {
    Image(U),
    Constant(T),
}

pub type MaterialValue1 = MaterialValue<f64, GrayImage>;
pub type MaterialValue3 = MaterialValue<Vector3<f64>, RgbImage>;

impl MaterialValue<f64, GrayImage> {
    pub fn get_value(&self, u: f64, v: f64) -> f64 {
        match *self {
            MaterialValue::Image(ref img) => {
                let width = img.width() as f64;
                let height = img.height() as f64;
                img.get_pixel((u * width) as u32, (v * height) as u32).0[0] as f64 / 255.0
            },
            MaterialValue::Constant(v) => v,
        }
    }

    pub fn from_file(name: &str) -> Self {
        let img = ImageReader::open(name).unwrap().decode().unwrap();
        let img = img.to_luma8();

        MaterialValue::Image(img)
    }

    pub fn from_constant(value: f64) -> Self {
        MaterialValue::Constant(value)
    }
}

impl MaterialValue<Vector3<f64>, RgbImage> {
    pub fn get_value(&self, u: f64, v: f64) -> Vector3<f64> {
        match *self {
            MaterialValue::Image(ref img) => {
                let width = img.width();
                let height = img.height();

                let x = (u * 100.0 * width as f64) as u32 % width;
                let y = (v * 100.0 * height as f64) as u32 % height;

                let pixel = img.get_pixel(x, y);
                let r = pixel.0[0] as f64 / 255.0;
                let g = pixel.0[1] as f64 / 255.0;
                let b = pixel.0[2] as f64 / 255.0;

                Vector3::new(r, g, b)
            },
            MaterialValue::Constant(ref v) => *v,
        }
    }

    pub fn from_file(name: &str) -> Self {
        let img = ImageReader::open(name).unwrap().decode().unwrap();
        let img = img.to_rgb8();

        MaterialValue::Image(img)
    }

    pub fn from_constant(value: Vector3<f64>) -> Self {
        MaterialValue::Constant(value)
    }
}

pub struct ImageMaterial {
    pub diffuse_strength: MaterialValue<f64, GrayImage>,
    pub color: MaterialValue<Vector3<f64>, RgbImage>,
    pub reflect_ratio: MaterialValue<f64, GrayImage>,
    pub refract_ratio: MaterialValue<f64, GrayImage>,
    pub refract_index: MaterialValue<f64, GrayImage>,

    // pub scale: f64,
}

impl ImageMaterial {
    pub fn new(
        diffuse_strength: MaterialValue1,
        color: MaterialValue3,
        reflect_ratio: MaterialValue1,
        refract_ratio: MaterialValue1,
        refract_index: MaterialValue1,
        // scale: f64,
    ) -> Self {
        ImageMaterial {
            diffuse_strength,
            color,
            refract_ratio,
            reflect_ratio,
            refract_index,
            // scale,
        }
    }
}

impl Material for ImageMaterial {
    // fn get_shininess(&self, u: f64, v: f64) -> f64 {
    //     self.shininess.get_value(u, v)
    // }

    fn get_color(&self, u: f64, v: f64) -> Vector3<f64> {
        self.color.get_value(u, v)
    }

    fn get_reflect_ratio(&self, u: f64, v: f64) -> f64 {
        self.reflect_ratio.get_value(u, v)
    }

    fn get_refract_ratio(&self, u: f64, v: f64) -> f64 {
        self.refract_ratio.get_value(u, v)
    }

    fn get_refract_index(&self, u: f64, v: f64) -> f64 {
        self.refract_index.get_value(u, v)
    }

    fn get_diffuse_strength(&self, u: f64, v: f64) -> f64 {
        self.diffuse_strength.get_value(u, v)
    }

    fn get_specular_strength(&self, u: f64, v: f64) -> f64 {
        self.reflect_ratio.get_value(u, v)
    }
}