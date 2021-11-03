use crate::ray::Ray;
use crate::object::{IntersectResult, Object};
use crate::light::Light;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectResult {
        let mut min_dis = f64::INFINITY;
        let mut result: Option<IntersectResult> = None;
        for obj in self.objects.iter() {
            let r = obj.intersect(&ray);
            if r.is_intersect {
                if r.dis < min_dis {
                    min_dis = r.dis;
                    result = Some(r);
                }
            }
        }

        result.unwrap_or(IntersectResult::no_intersect())
    }
}