use std::result::Result;
use glium::{VertexBuffer, GlObject, Program};
use glium::backend::Facade;
use ocl::{Buffer, ProQue, Context, Program as ClProgram};
use ocl::prm::Float3;
use ocl::core::MEM_READ_WRITE;
use point::Point;

const VERTEX_SRC: &'static str = include_str!("shaders/default.vert");
const FRAGMENT_SRC: &'static str = include_str!("shaders/default.frag");
const GEOMETRY_SRC: &'static str = include_str!("shaders/default.geom");
const PARTICLES_KERN_SRC: &'static str = include_str!("kernels/particles.cl");

pub type PartResult<T> = Result<T, &'static str>;

#[derive(Copy, Clone)]
pub struct Position {
    position: [f32; 4]
}

#[derive(Copy, Clone)]
pub struct Velocity {
    velocity: [f32; 4]
}

implement_vertex!(Position, position);
implement_vertex!(Velocity, velocity);

struct GlSide {
    positions: VertexBuffer<Position>,
    velocities: VertexBuffer<Velocity>,
    program: Program
}

struct Animation {
    from: Buffer<Float3>,
    to: Buffer<Float3>,
    duration: f32
}

struct ClSide {
    positions: Buffer<Float3>,
    velocities: Buffer<Float3>,
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
            x if x > 3_000_032 => { return Err("Cannot emit more than 3 millions particles.") },
            _ => ()
        }

        let gl_side = GlSide {
            positions: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            velocities: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            program: Program::from_source(facade, VERTEX_SRC, FRAGMENT_SRC,
                        Some(GEOMETRY_SRC)).unwrap()
        };

        let prog_bldr = ClProgram::builder().src(PARTICLES_KERN_SRC);
        let device = context.devices().first().unwrap().clone();
        let proque = ProQue::builder().context(context).prog_bldr(prog_bldr)
                        .device(device).dims([quantity]).build().unwrap();

        let from = Buffer::builder()
                          .queue(proque.queue().clone())
                          .flags(MEM_READ_WRITE)
                          .dims([quantity])
                          .build().unwrap();

        let to = Buffer::builder()
                        .queue(proque.queue().clone())
                        .flags(MEM_READ_WRITE)
                        .dims([quantity])
                        .build().unwrap();

        let cl_side = ClSide {
            positions: Buffer::from_gl_buffer(proque.queue(), Some(MEM_READ_WRITE),
                        [quantity], gl_side.positions.get_id()).unwrap(),
            velocities: Buffer::from_gl_buffer(proque.queue(), Some(MEM_READ_WRITE),
                        [quantity], gl_side.velocities.get_id()).unwrap(),
            animation: Animation { from, to, duration: 0.0_f32, },
            proque
        };
        Ok(Particles { quantity, gl_side, cl_side })
    }

    pub fn init_rand_sphere_animation(&mut self, duration: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.animation.duration = duration;
        self.cl_side.proque.create_kernel("init_rand_sphere_animation").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn init_rand_cube_animation(&mut self, duration: f32) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.animation.duration = duration;
        self.cl_side.proque.create_kernel("init_rand_cube_animation").unwrap()
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

    pub fn positions(&self) -> &VertexBuffer<Position> {
        &self.gl_side.positions
    }

    pub fn program(&self) -> &Program {
        &self.gl_side.program
    }
}
