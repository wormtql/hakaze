use cgmath::Vector3;
use super::material::Material;

pub struct NaiveMaterial {
    // pub shininess: f64,
    pub color: Vector3<f64>,
    pub reflect_ratio: f64,
    pub refract_ratio: f64,
    pub refract_index: f64,
    pub diffuse_strength: f64,
    pub specular_strength: f64,
}

impl NaiveMaterial {
    pub fn new(
        color: Vector3<f64>,
        reflect_ratio: f64,
        refract_ratio: f64,
        refract_index: f64,
        diffuse_strength: f64,
        specular_strength: f64,
    ) -> NaiveMaterial {
        NaiveMaterial {
            color,
            reflect_ratio,
            refract_ratio,
            refract_index,
            diffuse_strength,
            specular_strength,
        }
    }
}

impl Default for NaiveMaterial {
    fn default() -> Self {
        NaiveMaterial::new(
            Vector3::new(0.5, 0.5, 0.5),
            0.2,
            0.0,
            0.6,
            0.2,
            0.05,
        )
    }
}

impl Material for NaiveMaterial {
    // fn get_shininess(&self, u: f64, v: f64) -> f64 {
    //     self.shininess
    // }

    // fn get_ambient(&self, u: f64, v: f64) -> f64 {
    //     self.ambient
    // }

    fn get_color(&self, u: f64, v: f64) -> Vector3<f64> {
        self.color
    }

    fn get_reflect_ratio(&self, u: f64, v: f64) -> f64 {
        self.reflect_ratio
    }

    fn get_refract_ratio(&self, u: f64, v: f64) -> f64 {
        self.refract_ratio
    }

    fn get_refract_index(&self, u: f64, v: f64) -> f64 {
        self.refract_index
    }

    fn get_diffuse_strength(&self, u: f64, v: f64) -> f64 {
        self.diffuse_strength
    }

    fn get_specular_strength(&self, y: f64, v: f64) -> f64 {
        self.specular_strength
    }
}