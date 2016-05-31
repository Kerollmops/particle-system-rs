use nalgebra::{PerspectiveMatrix3, Isometry3, Point3, Vector3, ToHomogeneous, Matrix4};

pub struct Camera {
    projection: PerspectiveMatrix3<f32>,
    view: Isometry3<f32>
}

impl Camera {
    pub fn new(ratio: f32) -> Camera {
        let eye_pos = Point3::new(1.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        Camera {
            projection: PerspectiveMatrix3::new(ratio, 60.0, 0.0, 1000.0),
            view: Isometry3::look_at_rh(&eye_pos, &target, &Vector3::new(0.0, 1.0, 0.0))
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        (*self.projection.as_matrix()) * self.view.to_homogeneous()
    }
}
