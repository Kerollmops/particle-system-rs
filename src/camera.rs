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
        let eye_pos = Point3::new(1.0, -0.25, -0.5);
        let target = Point3::new(0.0, 0.0, 0.0);
        let draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                // range: (0.0, 1.0), // not working ???
                .. Default::default()
            },
            .. Default::default()
        };
        // let draw_parameters = Default::default();

        Camera {
            projection: PerspectiveMatrix3::new(width / height, 60.0, 0.001, 100.0),
            view: Isometry3::look_at_rh(&eye_pos, &target, &Vector3::new(0.0, 1.0, 0.0)),
            screen: Screen{ width: width, height: height },
            no_indices: NoIndices(PrimitiveType::Points),
            draw_parameters: draw_parameters
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.screen.width / self.screen.height
    }

    pub fn draw<S: Surface, T: Surface>(&self, steps_frame: &mut S, final_frame: &mut T,
                                        particles: &Particles, time: f32) {
        let mut projection = self.projection;
        projection.set_znear_and_zfar(0.001, 0.1);
        let matrix = (*projection.as_matrix()) * self.view.to_homogeneous();

        let uniforms = uniform!{
            matrix: *matrix.as_ref(),
            circle_diameter: 0.002_f32,
            aspect_ratio: self.aspect_ratio(),
            time: time
        };

        final_frame.draw(particles.positions(), &self.no_indices, particles.program(),
            &uniforms, &self.draw_parameters).unwrap();
    }
}
