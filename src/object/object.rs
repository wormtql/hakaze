use std::fs;

use cgmath::prelude::*;
use cgmath::{Vector3, Vector2, Matrix4, Vector4, Rad, Matrix2, Matrix3};
use std::convert::Infallible;
use std::cmp::Ordering;

use crate::ray::ray::Ray;
use crate::material::{Material, NaiveMaterial};

#[derive(Clone, Debug)]
pub struct PointStruct {
    pub vertex_index: i32,
    pub texture_index: i32,
    pub normal_index: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub vertex: Vector3<f64>,
    pub texture: Vector2<f64>,
    pub normal: Vector3<f64>,
}

// #[derive(Debug)]
pub struct Face3<'a> {
    pub points: [Point; 3],
    pub object: &'a Object,
}

#[derive(Eq, PartialEq, Debug)]
pub enum IntersectDirection {
    Positive,
    Negative,
}

// #[derive(Debug)]
pub struct IntersectResult<'a> {
    pub point: Option<Point>,
    pub direction: IntersectDirection,
    pub is_intersect: bool,
    pub dis: f64,
    pub object: Option<&'a Object>,
}

impl<'a> IntersectResult<'a> {
    pub fn no_intersect() -> IntersectResult<'a> {
        IntersectResult {
            point: None,
            direction: IntersectDirection::Positive,
            is_intersect: false,
            dis: 0.0,
            object: None,
        }
    }
}

impl<'a> Face3<'a> {
    fn calc_xy(&self, point: &Vector3<f64>) -> (f64, f64) {
        let p1 = &self.points[0];
        let p2 = &self.points[1];
        let p3 = &self.points[2];

        let ab = p2.vertex - p1.vertex;
        let ac = p3.vertex - p1.vertex;
        let ap = point - p1.vertex;

        let mat = Matrix2::new(
            ab.x, ab.y,
            ac.x,ac.y
        ).invert().unwrap();
        let result = mat * Vector2::new(ap.x, ap.y);

        (result.x, result.y)
    }

    fn interpolate(&self, p: &Vector3<f64>) -> Vector2<f64> {
        let a = &self.points[0].vertex;
        let b = &self.points[1].vertex;
        let c = &self.points[2].vertex;

        let ab = b - a;
        let ac = c - a;
        let z = ac.cross(ab);

        let t = Matrix3::new(
            ab.x, ab.y, ab.z,
            ac.x, ac.y, ac.z,
            z.x, z.y, z.z,
        ).invert().unwrap();

        let new_coord = t * (p - a);
        let w2 = new_coord.x;
        let w3 = new_coord.y;
        let w1 = 1.0 - w2 - w3;
        //
        // let pa = p1 - point;
        // let pb = p2 - point;
        // let pc = p3 - point;
        //
        // let da = p1.distance(*point);
        // let db = p2.distance(*point);
        // let dc = p3.distance(*point);
        //
        // let sa = 0.5 * db * dc * (1.0 - (pb.dot(pc).powf(2.0))).sqrt();
        // let sb = 0.5 * da * dc * (1.0 - (pa.dot(pc).powf(2.0))).sqrt();
        // let sc = 0.5 * da * db * (1.0 - (pa.dot(pb).powf(2.0))).sqrt();
        //
        // let w1 = sa / (sa + sb + sc);
        // let w2 = sb / (sa + sb + sc);
        // let w3 = sc / (sa + sb + sc);

        // let w1 = ((p2.y - p3.y) * (point.x - p3.x) + (p3.x - p2.x) * (point.y - p2.x) * (point.y - p3.y))
        //     / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));
        // let w2 = ((p3.y - p1.y) * (point.x - p3.x) + (p1.x - p3.x) * (point.y - p3.y))
        //     / ((p2.y - p3.y) * (p1.x - p3.x) + (p3.x - p2.x) * (p1.y - p3.y));
        // let w3 = 1.0 - w1 - w2;
        // println!("{:?}, {:?}, {:?}", w1, w2, w3);

        w1 * self.points[0].texture + w2 * self.points[1].texture + w3 * self.points[2].texture
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectResult<'a> {
        let p1 = &self.points[0];
        let p2 = &self.points[1];
        let p3 = &self.points[2];

        let t1 = p1.vertex - p3.vertex;
        let t2 = p2.vertex - p3.vertex;
        let normal = t1.cross(t2);

        let normal_x_d = normal.dot(ray.dir.clone());
        if normal_x_d.abs() < 1e-6 {
            // parallel
            return IntersectResult::no_intersect();
        }

        let d = -normal.dot(p1.vertex.clone());

        let t = -(normal.dot(ray.pos.clone()) + d) / (normal.dot(ray.dir.clone()));
        if t < 1e-6 {
            return IntersectResult::no_intersect();
        }
        let new_point = ray.pos + t * ray.dir;

        // check if new point is in the triangle
        let mut flag = (p2.vertex - p1.vertex).cross(new_point - p2.vertex).dot(normal) > 0.0;
        flag = flag && (p3.vertex - p2.vertex).cross(new_point - p3.vertex).dot(normal) > 0.0;
        flag = flag && (p1.vertex - p3.vertex).cross(new_point - p1.vertex).dot(normal) > 0.0;
        if !flag {
            return IntersectResult::no_intersect();
        }

        // let (x, y) = self.calc_xy(&new_point);
        // let new_texture = -(x + y) * p1.texture + x * p2.texture + y * p3.texture;
        // let new_texture = p1.texture.clone();
        let new_texture = self.interpolate(&new_point);
        // println!("{:?}", new_texture);
        let new_normal = p1.normal.clone();

        let direction = if ray.dir.dot(p1.normal.clone()) < 0.0 {
            IntersectDirection::Positive
        } else {
            IntersectDirection::Negative
        };
        IntersectResult {
            point: Some(Point {
                vertex: new_point,
                texture: new_texture,
                normal: new_normal,
            }),
            direction,
            is_intersect: true,
            dis: t,
            object: Some(&self.object),
        }
    }
}

pub struct FaceIter<'a> {
    next: usize,
    object: &'a Object,
    model_matrix: Matrix4<f64>,
    normal_matrix: Matrix4<f64>,
    // scale_factor: f64,
}

impl<'a> Iterator for FaceIter<'a> {
    type Item = Face3<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.object.faces.len() {
            return None;
        }
        let f = &self.object.faces[self.next];
        self.next += 1;

        let mut points: Vec<Point> = Vec::new();
        for p in f.points.iter().take(3) {
            let texture = &self.object.textures[p.texture_index as usize - 1];
            let vertex = &self.object.vertices[p.vertex_index as usize - 1];
            let new_vertex = Vector4::new(vertex[0], vertex[1], vertex[2], 1.0);
            let new_vertex: Vector4<f64> = self.model_matrix * new_vertex;
            let normal = &self.object.normals[p.normal_index as usize - 1];
            let new_normal = Vector4::new(normal[0], normal[1], normal[2], 1.0);
            let new_normal: Vector4<f64> = self.normal_matrix * new_normal;

            points.push(Point {
                vertex: Vector3::new(new_vertex[0], new_vertex[1], new_vertex[2]),
                texture: texture.clone(),
                normal: Vector3::new(new_normal[0], new_normal[1], new_normal[2]).normalize(),
            });
        }

        Some(Face3 {
            points: [points[0].clone(), points[1].clone(), points[2].clone()],
            object: &self.object,
        })
    }
}

#[derive(Debug)]
pub struct FaceStruct {
    pub points: Vec<PointStruct>,
}

// #[derive(Debug)]
pub struct Object {
    pub vertices: Vec<Vector3<f64>>,
    pub textures: Vec<Vector2<f64>>,
    pub normals: Vec<Vector3<f64>>,
    pub faces: Vec<FaceStruct>,

    pub rotate: Vector3<f64>,
    pub translate: Vector3<f64>,
    pub scale: Vector3<f64>,

    pub material: Box<dyn Material>,
}

impl PointStruct {
    pub fn new(vertex: i32, texture: i32, normal: i32) -> PointStruct {
        PointStruct {
            vertex_index: vertex,
            texture_index: texture,
            normal_index: normal,
        }
    }
}

impl Object {
    pub fn new() -> Object {
        Object {
            vertices: Vec::new(),
            textures: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new(),

            rotate: Vector3::new(0.0, 0.0, 0.0),
            translate: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),

            material: Box::new(NaiveMaterial::default()) as Box<dyn Material>,
        }
    }

    pub fn get_model_matrix(&self) -> Matrix4<f64> {
        let rot_x: Matrix4<f64> = Matrix4::from_angle_x(Rad(self.rotate[0]));
        let rot_y: Matrix4<f64> = Matrix4::from_angle_y(Rad(self.rotate[1]));
        let rot_z: Matrix4<f64> = Matrix4::from_angle_z(Rad(self.rotate[2]));
        let translate: Matrix4<f64> = Matrix4::from_translation(self.translate.clone());
        let scale: Matrix4<f64> = Matrix4::from_nonuniform_scale(self.scale[0], self.scale[1], self.scale[2]);

        // scale * rot_z * rot_y * rot_x * translate
        translate * rot_x * rot_y * rot_z * scale
    }

    pub fn get_rotate_matrix(&self) -> Matrix4<f64> {
        let rot_x: Matrix4<f64> = Matrix4::from_angle_x(Rad(self.rotate[0]));
        let rot_y: Matrix4<f64> = Matrix4::from_angle_y(Rad(self.rotate[1]));
        let rot_z: Matrix4<f64> = Matrix4::from_angle_z(Rad(self.rotate[2]));

        rot_x * rot_y * rot_z
        // rot_z * rot_y * rot_x
    }

    pub fn scale_uniform(&mut self, value: f64) -> &mut Object {
        self.scale[0] = value;
        self.scale[1] = value;
        self.scale[2] = value;

        self
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.scale[0] = x;
        self.scale[1] = y;
        self.scale[2] = z;
    }

    pub fn rotate_x(&mut self, value: f64) {
        self.rotate[0] = value;
    }

    pub fn rotate_y(&mut self, value: f64) {
        self.rotate[1] = value;
    }

    pub fn rotate_z(&mut self, value: f64) {
        self.rotate[2] = value;
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64) {
        self.rotate[0] = x;
        self.rotate[1] = y;
        self.rotate[2] = z;
    }

    pub fn translate_x(&mut self, value: f64) -> &mut Object {
        self.translate[0] = value;

        self
    }

    pub fn translate_y(&mut self, value: f64) -> &mut Object {
        self.translate[1] = value;

        self
    }

    pub fn translate_z(&mut self, value: f64) -> &mut Object {
        self.translate[2] = value;

        self
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> &mut Object {
        self.translate[0] = x;
        self.translate[1] = y;
        self.translate[2] = z;

        self
    }

    pub fn from_file(filename: &str) -> Object {
        let contents = fs::read_to_string(filename).unwrap();

        let mut object = Object::new();
        for line in contents.lines() {
            if line.starts_with("v ") {
                let coords: Vec<f64> = line.split(" ")
                    .skip(1)
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect();
                let v = Vector3::new(coords[0], coords[1], coords[2]);
                object.add_vertex(v);
            } else if line.starts_with("vt ") {
                let coords: Vec<f64> = line.split(" ")
                    .skip(1)
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect();
                let v = Vector2::new(coords[0], coords[1]);
                object.add_texture(v);
            } else if line.starts_with("vn ") {
                let coords: Vec<f64> = line.split(" ")
                    .skip(1)
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect();
                let v = Vector3::new(coords[0], coords[1], coords[2]);
                object.add_normal(v);
            } else if line.starts_with("f ") {
                let points_str: Vec<&str> = line.split(" ")
                    .skip(1)
                    .collect();
                let mut points: Vec<PointStruct> = Vec::new();
                for &p in points_str.iter() {
                    let values: Vec<i32> = p.split("/").map(|s| s.parse::<i32>().unwrap()).collect();
                    points.push(PointStruct::new(values[0], values[1], values[2]));
                }

                // make triangles
                for i in 1..points.len() - 1 {
                    let p1 = points[0].clone();
                    let p2 = points[i].clone();
                    let p3 = points[i + 1].clone();
                    let face = FaceStruct {
                        points: vec![p1, p2, p3],
                    };

                    object.add_face(face);
                }
            }
        }

        object
    }

    pub fn add_vertex(&mut self, vertex: Vector3<f64>) {
        self.vertices.push(vertex);
    }

    pub fn add_texture(&mut self, texture: Vector2<f64>) {
        self.textures.push(texture);
    }

    pub fn add_normal(&mut self, normal: Vector3<f64>) {
        self.normals.push(normal);
    }

    pub fn add_face(&mut self, face: FaceStruct) {
        self.faces.push(face);
    }

    pub fn faces_iter(&self) -> FaceIter {
        FaceIter {
            next: 0,
            object: &self,
            model_matrix: self.get_model_matrix(),
            normal_matrix: self.get_rotate_matrix(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectResult {
        let mut min_dis = f64::INFINITY;
        let mut result: Option<IntersectResult> = None;
        for face in self.faces_iter() {
            let r = face.intersect(&ray);
            if r.is_intersect {
                if r.dis < min_dis {
                    min_dis = r.dis;
                    result = Some(r);
                }
            }
        }

        result.unwrap_or(IntersectResult::no_intersect())
    }

    pub fn set_material(&mut self, material: Box<dyn Material>) -> &mut Object {
        self.material = material;

        self
    }
}