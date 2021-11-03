use cgmath::{Vector3, InnerSpace, MetricSpace};
use crate::light::Light;
use crate::ray::Ray;

pub struct PointLight {
    pub pos: Vector3<f64>,
    pub color: Vector3<f64>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
}

impl PointLight {
    pub fn new(pos: Vector3<f64>, color: Vector3<f64>, ambient: f64, diffuse: f64, specular: f64) -> PointLight {
        PointLight {
            pos,
            color,
            ambient,
            diffuse,
            specular,
        }
    }
}

impl Light for PointLight {
    fn get_ray(&self, point: Vector3<f64>) -> Ray {
        Ray {
            pos: point.clone(),
            dir: (self.pos - point).normalize(),
        }
    }

    fn get_color(&self, point: Vector3<f64>) -> Vector3<f64> {
        self.color.clone()
        // let temp = point.normalize();
        // Vector3::new(temp.x.abs(), temp.y.abs(), temp.z.abs())
    }

    fn is_blocked(&self, object_point: Vector3<f64>, intersect_point: Vector3<f64>) -> bool {
        let d1 = object_point.distance(intersect_point);
        let d2 = object_point.distance(self.pos);

        d1 < d2
    }

    fn get_ambient_strength(&self, point: Vector3<f64>) -> f64 {
        self.ambient
    }

    fn get_diffuse_strength(&self, point: Vector3<f64>) -> f64 {
        self.diffuse
    }

    fn get_specular_strength(&self, point: Vector3<f64>) -> f64 {
        self.specular
    }
}