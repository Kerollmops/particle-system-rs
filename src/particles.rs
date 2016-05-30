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

struct Animation {
    from: Buffer<ClFloat3>,
    to: Buffer<ClFloat3>,
    duration: f32
}

struct ClSide {
    positions: Buffer<ClFloat3>,
    velocities: Buffer<ClFloat3>,
    animation: Animation,
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
            animation: Animation {
                from: Buffer::new(&proque, Some(MEM_READ_WRITE), [quantity], None).unwrap(),
                to: Buffer::new(&proque, Some(MEM_READ_WRITE), [quantity], None).unwrap(),
                duration: 0.0_f32,
            },
            proque: proque
        };
        Ok(Particles {
            quantity: quantity,
            gl_side: gl_side,
            cl_side: cl_side
        })
    }

    pub fn init_sphere_animation(&mut self, duration: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.animation.duration = duration;
        self.cl_side.proque.create_kernel("init_sphere_animation").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn init_cube_animation(&mut self, duration: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.animation.duration = duration;
        self.cl_side.proque.create_kernel("init_cube_animation").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn update_animation(&mut self, time: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel("update_animation").unwrap()
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.positions)
            .arg_scl(time)
            .arg_scl(self.cl_side.animation.duration)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
    }

    pub fn update_gravitation(&mut self, gravity_point: Point, t: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel("update_gravitation").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .arg_vec(gravity_point)
            .arg_scl(t)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn draw(&self, frame: &mut Frame) {
        let indices = NoIndices(PrimitiveType::Points);
        frame.draw(&self.gl_side.positions, &indices, &self.gl_side.program,
            &EmptyUniforms, &Default::default()).unwrap();
        frame.set_finish().unwrap();
    }
}
