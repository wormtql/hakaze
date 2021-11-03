use cgmath::Vector3;

pub trait Material {
    // fn get_shininess(&self, u: f64, v: f64) -> f64;

    // fn get_diffuse(&self, u: f64, v: f64) -> f64;

    // fn get_ambient(&self, u: f64, v: f64) -> f64;

    fn get_color(&self, u: f64, v: f64) -> Vector3<f64>;

    fn get_reflect_ratio(&self, u: f64, v: f64) -> f64;

    fn get_refract_ratio(&self, u: f64, v: f64) -> f64;

    fn get_refract_index(&self, u: f64, v: f64) -> f64;

    fn get_diffuse_strength(&self, u: f64, v: f64) -> f64;

    fn get_specular_strength(&self, y: f64, v: f64) -> f64;
}

// pub trait MaterialValue<T> {
//     fn get_value(&self) -> T;
// }
//
// pub struct ConstantMaterialValue<T> {
//     value: T,
// }
//
// impl<T> ConstantMaterialValue<T> {
//     pub fn new(value: T) -> ConstantMaterialValue<T> {
//         ConstantMaterialValue {
//             value,
//         }
//     }
// }
//
// impl<T: Copy> MaterialValue<T> for ConstantMaterialValue<T> {
//     fn get_value(&self) -> T {
//         self.value
//     }
// }
//
// pub struct ImageMaterialValue