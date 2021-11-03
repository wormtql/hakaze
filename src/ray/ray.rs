use cgmath::prelude::*;
use cgmath::Vector3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new_nz() -> Ray {
        Ray {
            pos: Vector3::new(0.0, 0.0, 0.0),
            dir: Vector3::new(0.0, 0.0, -1.0),
        }
    }
}