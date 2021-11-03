use cgmath::{Matrix4, Vector3, Point3, Rad, Vector4, SquareMatrix, InnerSpace, Transform, Matrix};
use crate::ray::Ray;

pub trait Camera {
    fn view(&self) -> Matrix4<f64>;

    fn proj(&self) -> Matrix4<f64>;

    // fn to_world(&self) -> Vector3<f64>;

    fn get_ray(&self, x: f64, y: f64) -> Ray;

    fn map(&self, pos: Vector3<f64>) -> Vector3<f64> {
        let temp = Vector4::new(pos.x, pos.y, pos.z, 1.0);
        let view = self.view();
        let proj = self.proj();

        // let mut after = (proj * view).transform_vector(pos);
        let after = proj * view * temp;
        // println!("{:?}", after);
        Vector3::new(after.x, after.y, after.z) / after.w
        // after
    }
}

#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
    pub eye: Point3<f64>,
    pub center: Point3<f64>,
    pub up: Vector3<f64>,

    pub fovy: f64,
    pub aspect: f64,
    pub near: f64,
    pub far: f64,
}

impl PerspectiveCamera {
    pub fn new(fovy: f64, aspect: f64, near: f64, far: f64) -> PerspectiveCamera {
        PerspectiveCamera {
            eye: Point3::new(0.0, 0.0, 0.0),
            center: Point3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),

            fovy,
            aspect,     // width / height
            near,
            far,
        }
    }

    pub fn set_eye(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.eye = Point3::new(x, y, z);

        self
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.center = Point3::new(x, y, z);

        self
    }

    pub fn set_up(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.up = Vector3::new(x, y, z);

        self
    }
}

impl Camera for PerspectiveCamera {
    fn view(&self) -> Matrix4<f64> {
        Matrix4::look_at_rh(self.eye, self.center, self.up)
    }

    fn proj(&self) -> Matrix4<f64> {
        cgmath::perspective(Rad(self.fovy), self.aspect, self.near, self.far)
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray {
        // let proj = self.proj();
        // let proj_inv = proj.invert().unwrap();
        let view = self.view();
        let view_inv = view.invert().unwrap();

        let f = (self.fovy / 2.0).tan();
        let world_x = x * self.aspect * self.near * f;
        let world_y = y * self.near * f;
        let world = Vector4::new(world_x, world_y, -self.near, 1.0);

        let world = view_inv * world;
        let world = Vector3::new(world.x, world.y, world.z);

        let dir = world.clone() - Vector3::new(self.eye[0], self.eye[1], self.eye[2]);
        let dir = dir.normalize();

        Ray {
            pos: world,
            dir,
        }
    }
}

#[cfg(test)]
mod camera_test {
    use super::PerspectiveCamera;
    use crate::camera::Camera;
    use crate::ray::Ray;
    use cgmath::{Vector3, Matrix4, Vector4};

    #[test]
    fn test_perspective_camera_1() {
        let camera = PerspectiveCamera::new(std::f64::consts::PI / 2.0, 1.0, 1.0, 2.0);
        let ray = camera.get_ray(0.0, 0.0);
        assert_eq!(ray, Ray {
            pos: Vector3::new(0.0, 0.0, -1.0),
            dir: Vector3::new(0.0, 0.0, -1.0),
        });
    }

    #[test]
    fn test_perspective_camera_2() {
        let camera = PerspectiveCamera::new(std::f64::consts::PI / 2.0, 1.0, 1.0, 3.0);
        let ray = camera.get_ray(0.0, 0.0);
        assert_eq!(ray, Ray {
            pos: Vector3::new(0.0, 0.0, -1.0),
            dir: Vector3::new(0.0, 0.0, -1.0),
        });
    }

    #[test]
    fn test_perspective_camera_3() {
        let camera = PerspectiveCamera::new(std::f64::consts::PI / 2.0, 2.0, 2.0, 4.0);
        let ray = camera.get_ray(0.0, 0.0);
        assert_eq!(ray, Ray {
            pos: Vector3::new(0.0, 0.0, -2.0),
            dir: Vector3::new(0.0, 0.0, -1.0),
        });
    }

    // #[test]
    // fn test_perspective_camera_4() {
    //     let camera = PerspectiveCamera::new(std::f64::consts::PI / 2.0, 1.0, 2.0, 4.0);
    //     let ray = camera.get_ray(0.5, 0.5);
    //     assert_eq!(ray, Ray {
    //         pos: Vector3::new(0.0, 0.0, -2.0),
    //         dir: Vector3::new(0.0, 0.0, -1.0),
    //     });
    // }

    // #[test]
    // fn test_perspective_camera_4() {
    //     let mut camera = PerspectiveCamera::new(std::f64::consts::PI / 2.0, 1.0, 1.0, 2.0);
    //     // camera.set_eye(0.0, 0.0, 1.0);
    //     let p = Vector3::new(1.0, 0.0, -2.0);
    //     let pp = camera.map(p);
    //
    //     assert_eq!(pp, Vector3::new(0.0, 0.0, 1.0));
    // }

    // #[test]
    // fn test_perspective_camera_5() {
    //     let a = Matrix4::new(
    //         1.0, 2.0, 3.0, 4.0,
    //         0.0, 0.0, 0.0, 0.0,
    //         0.0, 0.0, 0.0, 0.0,
    //         0.0, 0.0, 0.0, 0.0,
    //     );
    //     let v = Vector4::new(1.0, 1.0, 1.0, 1.0);
    //     let product = a * v;
    //     assert_eq!(product, Vector4::new(10.0, 0.0, 0.0, 0.0));
    // }
}