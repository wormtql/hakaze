use std::f64::consts::PI;

use ray_tracing::object::{Object};
use ray_tracing::ray::Ray;
use ray_tracing::camera::{PerspectiveCamera, Camera};
use ray_tracing::scene::Scene;
use ray_tracing::tracing::{BinaryTracing, Tracing, MyTracing};
use ray_tracing::light::{PointLight, Light};
use cgmath::Vector3;
use ray_tracing::material::{ChessBoardMaterial, Material, ImageMaterial, MaterialValue1, MaterialValue3, NaiveMaterial};
use ray_tracing::material::image_material::MaterialValue;
use image::GrayImage;


fn set_up_objects(scene: &mut Scene) {
    // add objects
    let mut obj = Object::from_file("models/cube.obj");
    obj.scale_uniform(1.5);
    obj.translate_z(-6.0);
    obj.translate_y(1.0).translate_x(1.0);
    obj.rotate_x(2.0);
    obj.rotate_y(1.0);
    obj.set_material(Box::new(NaiveMaterial::new(
        Vector3::new(1.0, 0.0, 0.3),
        0.5,
        0.9,
        0.6,
        0.03,
        1.0,
    )) as Box<dyn Material>);
    // obj.set_material(Box::new(ChessBoardMaterial::new()) as Box<dyn Material>);

    let mut obj2 = Object::from_file("models/forlai.obj");
    obj2.scale_uniform(1.5).translate_x(-1.0).translate_y(1.0).translate_z(-5.0);
    obj2.rotate_x(1.0);
    obj2.rotate_y(1.0);
    obj2.set_material(Box::new(NaiveMaterial::new(
        Vector3::new(0.5, 0.1, 0.8),
        0.5,
        0.9,
        0.6,
        0.03,
        1.0,
    )) as Box<dyn Material>);

    let mut obj3 = Object::from_file("models/plane.obj");
    obj3.scale_uniform(200.0);
    // obj3.set_material(Box::new(ChessBoardMaterial::new()) as Box<dyn Material>);
    obj3.set_material(Box::new(ImageMaterial::new(
        MaterialValue1::from_constant(0.9),
        // MaterialValue1::from_file("images/TexturesCom_Metal_Threadplate3_1K_roughness.tif"),
        MaterialValue3::from_file("images/TexturesCom_Metal_Threadplate3_1K_albedo.tif"),
        // MaterialValue1::from_file("images/TexturesCom_Metal_Threadplate3_1K_metallic.tif"),
        MaterialValue1::from_constant(0.01),
        MaterialValue::Constant(0.0),
        MaterialValue::Constant(0.0),
    )) as Box<dyn Material>);
    obj3.translate_y(-2.0);

    let mut obj4 = Object::from_file("models/cube.obj");
    obj4.scale_uniform(1.0);
    obj4.translate_x(10.0);
    obj4.translate_z(-50.0);
    obj4.translate_y(10.0);
    // obj4.rotate_x(2.0);
    // obj4.rotate_y(1.0);

    scene.add_object(obj);
    scene.add_object(obj2);
    scene.add_object(obj3);
    // scene.add_object(obj4);
}

fn main() {
    let render_width = 500;
    let render_height = 500;

    let mut scene: Scene = Scene::new();
    set_up_objects(&mut scene);


    // add lights
    let light1 = PointLight::new(
        Vector3::new(0.0, 10.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        0.5,
        1.0,
        1.0,
    );
    let light2 = PointLight::new(
        Vector3::new(10.0, 20.0, -50.0),
        Vector3::new(1.0, 1.0, 1.0),
        0.5,
        1.0,
        1.0,
    );
    let light3 = PointLight::new(
        Vector3::new(-10.0, 1.0, -50.0),
        Vector3::new(1.0, 1.0, 1.0),
        0.5,
        0.5,
        1.0,
    );
    scene.add_light(Box::new(light1) as Box<dyn Light>);
    scene.add_light(Box::new(light2) as Box<dyn Light>);
    scene.add_light(Box::new(light3) as Box<dyn Light>);


    let mut camera = PerspectiveCamera::new(PI / 2.0, 1.0, 0.1, 10.0);
    // camera.set_eye(0.0, 2.0, 0.0);

    // let tracing = BinaryTracing::new(&scene, (&camera) as (&dyn Camera));
    let tracing = MyTracing::new(&scene, (&camera) as (&dyn Camera));
    let img = tracing.trace(render_width, render_height);

    img.save("test.png");
    // let ray = Ray::new_nz();

    // let result = obj.intersect(&ray);
    // println!("{:?}", result);
    // println!("{:?}", obj);
}
