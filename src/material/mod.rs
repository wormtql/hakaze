pub mod material;
pub mod naive_material;
pub mod chess_board_material;
pub mod image_material;

pub use material::Material;
// pub use material::MaterialValue;
// pub use material::ConstantMaterialValue;
pub use naive_material::NaiveMaterial;
pub use chess_board_material::ChessBoardMaterial;
pub use image_material::ImageMaterial;
pub use image_material::MaterialValue1;
pub use image_material::MaterialValue3;