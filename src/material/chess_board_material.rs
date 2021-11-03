use crate::material::Material;
use cgmath::Vector3;

pub struct ChessBoardMaterial {

}

impl ChessBoardMaterial {
    pub fn new() -> ChessBoardMaterial {
        ChessBoardMaterial {}
    }
}

impl Material for ChessBoardMaterial {
    // fn get_shininess(&self, u: f64, v: f64) -> f64 {
    //     256.0
    // }

    fn get_color(&self, u: f64, v: f64) -> Vector3<f64> {
        let x = (u * 100.0) as u32;
        let y = (v * 100.0) as u32;

        if (x + y) % 2 == 0 {
            Vector3::new(0.0, 0.0, 0.0)
        } else {
            Vector3::new(1.0, 1.0, 1.0)
        }
    }

    fn get_reflect_ratio(&self, u: f64, v: f64) -> f64 {
        0.1
    }

    fn get_refract_ratio(&self, u: f64, v: f64) -> f64 {
        0.0
    }

    fn get_refract_index(&self, u: f64, v: f64) -> f64 {
        0.0
    }

    fn get_diffuse_strength(&self, u: f64, v: f64) -> f64 {
        0.5
    }

    fn get_specular_strength(&self, y: f64, v: f64) -> f64 {
        0.5
    }
}