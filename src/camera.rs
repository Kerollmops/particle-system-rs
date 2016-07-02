use nalgebra::{PerspectiveMatrix3, Isometry3, Point3, Vector3, Matrix4, Eye};
use glium::{DrawParameters, Depth, DepthTest, VertexBuffer, IndexBuffer,
            Program, Texture2d, Frame};
use glium::Surface;
use glium::texture::depth_texture2d::DepthTexture2d;
use glium::backend::glutin_backend::GlutinFacade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::index::{IndicesSource, NoIndices, PrimitiveType};
use glium::uniforms::Uniforms;
use glium::backend::Facade;
use glium::uniforms::{Sampler, MagnifySamplerFilter};
use time::Duration;
use nalgebra::{ToHomogeneous, Identity};
use particles::Particles;

const CIRCLES_VERT: &'static str = include_str!("shaders/circles.vert");
const CIRCLES_FRAG: &'static str = include_str!("shaders/circles.frag");
const CIRCLES_GEOM: &'static str = include_str!("shaders/circles.geom");

const BLUR_QUAD_VERT: &'static str = include_str!("shaders/blur_quad.vert");
const BLUR_QUAD_FRAG: &'static str = include_str!("shaders/blur_quad.frag");

// const BACKGROUND: (f32, f32, f32, f32) = (0.17578, 0.17578, 0.17578, 1.0); // sRGB
const BACKGROUND: (f32, f32, f32, f32) = (0.026, 0.026, 0.026, 1.0); // ???
// const BACKGROUND: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
// const BACKGROUND: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

struct Screen {
    width: f32,
    height: f32
}

struct BlurQuad<'a> {
    draw_parameters: DrawParameters<'a>,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
    program: Program
}

struct DepthSteps<'a> {
    draw_parameters: DrawParameters<'a>,
    indices: NoIndices,
    program: Program,
    color_texture: Texture2d,
    depth_texture: DepthTexture2d
}

pub struct Camera<'a> {
    projection: PerspectiveMatrix3<f32>,
    view: Isometry3<f32>,
    blur_quad: BlurQuad<'a>,
    depth_steps: DepthSteps<'a>,
    screen: Screen,
}

impl<'a> Camera<'a> {
    pub fn new<F: Facade>(facade: &F, width: f32, height: f32) -> Camera<'a> {
        let eye_pos = Point3::new(1.0, -0.25, -0.5);
        let target = Point3::new(0.0, 0.0, 0.0);

        // building the index buffer
        let bq_index_buffer = IndexBuffer::new(facade, PrimitiveType::TriangleStrip,
                                &[1 as u16, 2, 0, 3]).unwrap();

        // building the vertex buffer, which contains all the vertices that we will draw
        let bq_vertex_buffer = VertexBuffer::new(facade, &[
                    Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                    Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                    Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                    Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] } ]).unwrap();

        let blur_quad = BlurQuad {
            draw_parameters: Default::default(), // overwrite mode
            vertex_buffer: bq_vertex_buffer, // give this each time drawing
            indices: bq_index_buffer,
            program: Program::from_source(facade, BLUR_QUAD_VERT, BLUR_QUAD_FRAG, None).unwrap()
        };

        let dt_draw_parameters = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLess,
                write: true,
                // range: (0.0, 1.0), // not working ???
                .. Default::default()
            },
            .. Default::default()
        };

        let color_texture = Texture2d::empty(facade, width as u32, height as u32).unwrap();
        let depth_texture = DepthTexture2d::empty(facade, width as u32, height as u32).unwrap();

        let depth_steps = DepthSteps {
            draw_parameters: dt_draw_parameters,
            indices: NoIndices(PrimitiveType::Points),
            program: Program::from_source(facade, CIRCLES_VERT, CIRCLES_FRAG, Some(CIRCLES_GEOM)).unwrap(),
            color_texture: color_texture,
            depth_texture: depth_texture
        };

        Camera {
            projection: PerspectiveMatrix3::new(width / height, 60.0, 0.001, 100.0),
            view: Isometry3::look_at_rh(&eye_pos, &target, &Vector3::new(0.0, 1.0, 0.0)),
            screen: Screen{ width: width, height: height },
            blur_quad: blur_quad,
            depth_steps: depth_steps
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.screen.width / self.screen.height
    }

    pub fn draw(&self, facade: &GlutinFacade, particles: &Particles, time: Duration) {
        let mut projection = self.projection;
        let color_texture = &self.depth_steps.color_texture;
        let depth_texture = &self.depth_steps.depth_texture;
        let mut frame_texture = SimpleFrameBuffer::with_depth_buffer(facade,
                            color_texture, depth_texture).unwrap();

        // for pat in expr {
            // projection.set_znear_and_zfar(0.001, 0.1);
            let matrix = (*projection.as_matrix()) * self.view.to_homogeneous();
            let circles_uniforms = uniform!{
                matrix: *matrix.as_ref(),
                circle_diameter: 0.002_f32,
                aspect_ratio: self.aspect_ratio(),
                time: time.num_milliseconds() as f32
            };
            frame_texture.clear_color_srgb_and_depth(BACKGROUND, 1.0);
            frame_texture.draw(particles.positions(), &self.depth_steps.indices,
                &self.depth_steps.program, &circles_uniforms, &self.depth_steps.draw_parameters).unwrap();
        // }

        let tex = Sampler::new(&self.depth_steps.color_texture).magnify_filter(MagnifySamplerFilter::Nearest);

        let blur_quad_uniforms = uniform! {
            matrix: *Matrix4::<f32>::new_identity(4).as_ref(),
            aspect_ratio: self.aspect_ratio(),
            // tex: &self.depth_steps.color_texture,
            tex: tex,
            resolution: [self.screen.width, self.screen.height],
            time: time.num_milliseconds() as f32
        };

        let mut frame = (*facade).draw();
        // frame.clear_color_srgb_and_depth(BACKGROUND, 1.0);
        frame.draw(&self.blur_quad.vertex_buffer, &self.blur_quad.indices,
            &self.blur_quad.program, &blur_quad_uniforms, &self.blur_quad.draw_parameters).unwrap();
        frame.finish().unwrap();
    }
}
