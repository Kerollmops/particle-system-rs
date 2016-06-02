use nalgebra::{PerspectiveMatrix3, Isometry3, Point3, Vector3, Matrix4};
use glium::{DrawParameters, Depth, DepthTest, Surface};
use glium::index::{NoIndices, PrimitiveType};
use nalgebra::ToHomogeneous;
use particles::Particles;
use glium::Frame;

struct Screen {
    width: f32,
    height: f32
}

pub struct Camera<'a> {
    projection: PerspectiveMatrix3<f32>,
    view: Isometry3<f32>,
    draw_parameters: DrawParameters<'a>,
    no_indices: NoIndices,
    screen: Screen,
}

impl<'a> Camera<'a> {
    pub fn new(width: f32, height: f32) -> Camera<'a> {
        let eye_pos = Point3::new(1.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };
        Camera {
            projection: PerspectiveMatrix3::new(width / height, 60.0, 0.001, 1000.0),
            view: Isometry3::look_at_rh(&eye_pos, &target, &Vector3::new(0.0, 1.0, 0.0)),
            screen: Screen{ width: width, height: height },
            no_indices: NoIndices(PrimitiveType::Points),
            draw_parameters: draw_parameters
        }
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        (*self.projection.as_matrix()) * self.view.to_homogeneous()
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.screen.width / self.screen.height
    }

    pub fn draw(&self, frame: &mut Frame, particles: &Particles) {
        let uniforms = uniform!{
            matrix: *self.matrix().as_ref(),
            circle_diameter: 0.002_f32,
            aspect_ratio: self.aspect_ratio()
        };
        frame.draw(particles.positions(), &self.no_indices, particles.program(),
            &uniforms, &self.draw_parameters).unwrap();
        frame.set_finish().unwrap();
    }
}
