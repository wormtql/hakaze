use cgmath::prelude::*;

use super::tracing::Tracing;
use image::{RgbImage, Rgb, ImageBuffer};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::object::IntersectDirection;
use cgmath::Vector3;

pub struct MyTracing<'a> {
    pub scene: &'a Scene,
    pub camera: &'a dyn Camera,

    pub max_depth: u32,
}

impl<'a> Tracing for MyTracing<'a> {
    fn trace(&self, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = RgbImage::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let x = (2.0 * i as f64 - width as f64) / width as f64;
                let y = (height as f64 - 2.0 * j as f64) / height as f64;
                let ray = self.camera.get_ray(x, y);

                let color = self.trace_helper(&ray, 0);
                let color = vec3_to_rgb(&color);
                img.put_pixel(i, j, color);
            }
        }

        img
    }
}

fn vec3_to_rgb(color: &Vector3<f64>) -> Rgb<u8> {
    let r = (color[0] * 255.0) as u8;
    let g = (color[1] * 255.0) as u8;
    let b = (color[2] * 255.0) as u8;

    Rgb([r, g, b])
}

fn reflect(a: Vector3<f64>, axis: Vector3<f64>) -> Vector3<f64> {
    let temp = a.dot(axis) * axis;
    2.0 * temp - a
}

fn refract(a: Vector3<f64>, n: Vector3<f64>, index: f64) -> Option<Vector3<f64>> {
    let cos_i = a.dot(n);
    let k = 1.0 - index * index * (1.0 - cos_i * cos_i);
    if k < 0.0 {
        None
    } else {
        Some(index * a - (index * cos_i + k.sqrt()) * n)
    }
}

impl<'a> MyTracing<'a> {
    pub fn new(scene: &'a Scene, camera: &'a dyn Camera) -> MyTracing<'a> {
        MyTracing {
            scene,
            camera,

            max_depth: 8
        }
    }

    fn trace_helper(&self, ray: &Ray, depth: u32) -> Vector3<f64> {
        if depth >= self.max_depth {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let intersect_result = self.scene.intersect(&ray);

        if !intersect_result.is_intersect {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let point = intersect_result.point.unwrap();
        let u = point.texture.x;
        let v = point.texture.y;
        // todo normal
        let normal = point.normal.clone();

        let collide_object = intersect_result.object.unwrap();

        let mut color: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
        let object_base_color = collide_object.material.get_color(u, v);

        let mut reflect_ratio = collide_object.material.get_reflect_ratio(u, v);
        let mut refract_ratio = collide_object.material.get_refract_ratio(u, v);
        let mut refract_index = collide_object.material.get_refract_index(u, v);
        let mut specular_strength = collide_object.material.get_specular_strength(u, v);
        let mut diffuse_strength = collide_object.material.get_diffuse_strength(u, v);

        // lights
        // if let IntersectDirection::Positive = intersect_result.direction {
            for light in self.scene.lights.iter() {
                let shadow_ray = light.get_ray(point.vertex.clone());
                // if shadow_ray.dir.dot(normal) < 0.0 {
                //     continue;
                // }
                let shadow_ray_intersect_result = self.scene.intersect(&shadow_ray);

                // let light_color = light.get_color(point.vertex.clone());
                let light_ambient_strength = light.get_ambient_strength(point.vertex.clone());
                let light_diffuse_strength = light.get_diffuse_strength(point.vertex.clone());
                let light_specular_strength = light.get_specular_strength(point.vertex.clone());

                let mut is_blocked = if !shadow_ray_intersect_result.is_intersect {
                    false
                } else {
                    let shadow_ray_intersect_pos = shadow_ray_intersect_result.point.unwrap().vertex;
                    light.is_blocked(point.vertex, shadow_ray_intersect_pos)
                };

                let light_color = if !is_blocked {
                    // ambient
                    // let ambient = light_color * light_ambient_strength;
                    // color += ambient.mul_element_wise(object_base_color);
                    light.get_color(point.vertex)
                } else if shadow_ray.dir.dot(normal) > 0.0 {
                    self.trace_helper(&shadow_ray, depth + 1)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                };

                // diffuse
                let diffuse = light_color * shadow_ray.dir.dot(normal.clone()).max(0.0) * diffuse_strength;

                // specular
                let half = (-ray.dir + shadow_ray.dir).normalize();
                // let shininess = collide_object.material.get_shininess(u, v);
                let shininess = 128.0;
                let specular = light_color * half.dot(normal.clone()).max(0.0).powf(shininess) * specular_strength;

                color += (diffuse + specular).mul_element_wise(object_base_color);
            }
        // }

        let reflect_dir = reflect(-ray.dir, normal).normalize();
        let refract_normal = match intersect_result.direction {
            IntersectDirection::Positive => normal,
            IntersectDirection::Negative => -normal,
        };
        refract_index = match intersect_result.direction {
            IntersectDirection::Positive => refract_index,
            IntersectDirection::Negative => 1.0 / refract_index,
        };
        let refract_dir = refract(ray.dir, refract_normal, refract_index);

        if let None = refract_dir {
            // 全反射
            reflect_ratio += refract_ratio;
            refract_ratio = 0.0;
        }

        // reflect
        let reflect_ray = Ray {
            pos: point.vertex,
            dir: reflect_dir,
        };

        let reflect_color = if reflect_ratio > 1e-6 {
            self.trace_helper(&reflect_ray, depth + 1) * reflect_ratio
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };
        color += reflect_color;

        // refract
        if refract_ratio > 1e-6 {
            let refract_ray = Ray {
                pos: point.vertex,
                dir: refract_dir.unwrap(),
            };

            let refract_color = self.trace_helper(&refract_ray, depth + 1) * refract_ratio;
            color += refract_color;
        }

        color * ((self.max_depth + 30 - depth) as f64) / (self.max_depth as f64 + 30.0)
    }
}