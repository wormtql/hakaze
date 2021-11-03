use image::RgbImage;
use crate::scene::Scene;
use crate::camera::Camera;

pub trait Tracing {
    fn trace(&self, width: u32, height: u32) -> RgbImage;
}