use std::result::Result;
use glium::{VertexBuffer, GlObject, Frame, Surface, Program};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::EmptyUniforms;
use glium::backend::Facade;
use cgmath::{PerspectiveFov, Rad};
use ocl::{Buffer, ProQue, Context, Program as ClProgram};
use ocl::aliases::ClFloat3;
use ocl::core::MEM_READ_WRITE;
use point::Point;

const VERTEX_SRC: &'static str = include_str!("shaders/default.vert");
const FRAGMENT_SRC: &'static str = include_str!("shaders/default.frag");
const PARTICLES_KERN_SRC: &'static str = include_str!("kernels/particles.cl");

pub type PartResult<T> = Result<T, &'static str>;

#[derive(Copy, Clone)]
struct Position {
    position: [f32; 4]
}

#[derive(Copy, Clone)]
struct Velocity {
    velocity: [f32; 4]
}

implement_vertex!(Position, position);
implement_vertex!(Velocity, velocity);

struct GlSide {
    positions: VertexBuffer<Position>,
    velocities: VertexBuffer<Velocity>,
    program: Program,
    persp_proj: PerspectiveFov<f32>
}

struct ClSide {
    positions: Buffer<ClFloat3>,
    velocities: Buffer<ClFloat3>,
    proque: ProQue
}

pub struct Particles {
    quantity: usize,
    gl_side: GlSide,
    cl_side: ClSide
}

impl Particles {
    pub fn new<F: Facade>(facade: &F, context: Context, quantity: usize) -> PartResult<Particles> {
        match quantity {
            0 => { return Err("Cannot emit zero particles.") },
            x if x > 3_000_000 => { return Err("Cannot emit more than 3 millions particles.") },
            _ => ()
        }
        let gl_side = GlSide {
            positions: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            velocities: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            program: Program::from_source(facade, VERTEX_SRC, FRAGMENT_SRC, None).unwrap(),
            persp_proj: PerspectiveFov { fovy: Rad { s: 60.0 }, aspect: 0.0f32, near: 0.1, far: 1000.0 } // TODO need to be computed ?
        };

        let prog_bldr = ClProgram::builder().src(PARTICLES_KERN_SRC);
        let device = context.devices().first().unwrap().clone();
        let proque = ProQue::builder().context(context).prog_bldr(prog_bldr)
                        .device(device).dims([quantity]).build().unwrap();

        let cl_side = ClSide {
            positions: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                [quantity], gl_side.positions.get_id()).unwrap(),
            velocities: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                [quantity], gl_side.velocities.get_id()).unwrap(),
            proque: proque
        };
        Ok(Particles {
            quantity: quantity,
            gl_side: gl_side,
            cl_side: cl_side
        })
    }

    fn acquire_buffers(&mut self) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();
    }

    fn release_buffers(&mut self) {
        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn init_sphere(&mut self) {
        self.acquire_buffers();

        self.cl_side.proque.create_kernel("init_sphere").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.release_buffers();
    }

    pub fn init_cube(&mut self) {
        self.acquire_buffers();

        self.cl_side.proque.create_kernel("init_cube").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.release_buffers();
    }

    pub fn update(&mut self, gravity_point: Point) {
        self.acquire_buffers();

        self.cl_side.proque.create_kernel("update").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .arg_vec(gravity_point)
            .enq().unwrap();

        self.release_buffers();
    }

    pub fn draw(&self, frame: &mut Frame) {
        let indices = NoIndices(PrimitiveType::Points);
        frame.draw(&self.gl_side.positions, &indices, &self.gl_side.program,
            &EmptyUniforms, &Default::default()).unwrap();
        frame.set_finish().unwrap();
    }
}
