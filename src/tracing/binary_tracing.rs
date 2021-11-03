use cgmath::prelude::*;

use super::tracing::Tracing;
use image::{RgbImage, Rgb, ImageBuffer};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::camera::Camera;

pub struct BinaryTracing<'a> {
    pub scene: &'a Scene,
    pub camera: &'a dyn Camera,
}

impl<'a> Tracing for BinaryTracing<'a> {
    fn trace(&self, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = RgbImage::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let x = (2.0 * i as f64 - width as f64) / width as f64;
                let y = (height as f64 - 2.0 * j as f64) / height as f64;
                let ray = self.camera.get_ray(x, y);

                let color = self.trace_helper(&ray);
                img.put_pixel(i, j, color);
            }
        }

        img
    }
}

impl<'a> BinaryTracing<'a> {
    pub fn new(scene: &'a Scene, camera: &'a dyn Camera) -> BinaryTracing<'a> {
        BinaryTracing {
            scene,
            camera,
        }
    }

    fn trace_helper(&self, ray: &Ray) -> Rgb<u8> {
        let intersect_result = self.scene.intersect(&ray);
        return if intersect_result.is_intersect {
            let point = intersect_result.point.unwrap();
            // let cos = point.normal.dot(ray.dir.clone()).abs();
            let cos = point.normal.normalize().dot(-ray.dir.clone());
            // println!("{}", cos);
            // Rgb([255, 0, 0])
            Rgb([(255.0 * cos) as u8, 0, 0])
        } else {
            Rgb([0, 0, 0])
        }
    }
}