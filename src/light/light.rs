use cgmath::Vector3;
use crate::ray::Ray;

pub trait Light {
    fn get_ray(&self, point: Vector3<f64>) -> Ray;

    fn get_color(&self, point: Vector3<f64>) -> Vector3<f64>;

    fn is_blocked(&self, object_point: Vector3<f64>, intersect_point: Vector3<f64>) -> bool;

    fn get_ambient_strength(&self, point: Vector3<f64>) -> f64;

    fn get_diffuse_strength(&self, point: Vector3<f64>) -> f64;

    fn get_specular_strength(&self, point: Vector3<f64>) -> f64;
}