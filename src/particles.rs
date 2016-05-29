use glium::{VertexBuffer, GlObject};
use glium::backend::Facade;
use ocl::{Buffer, ProQue, Program};
use ocl::core::MEM_READ_WRITE;
use point::Point;

#[derive(Copy, Clone)]
struct Position {
    position: [f32; 3]
}

#[derive(Copy, Clone)]
struct Velocity {
    velocity: [f32; 3]
}

implement_vertex!(Position, position);
implement_vertex!(Velocity, velocity);

struct GlSide {
    positions: VertexBuffer<Position>,
    velocities: VertexBuffer<Velocity>
}

struct ClSide {
    positions: Buffer<f32>,
    velocities: Buffer<f32>,
    proque: ProQue
}

pub struct Particles {
    quantity: usize,
    gl_side: GlSide,
    cl_side: ClSide
}

impl Particles {
    pub fn new<F: Facade>(facade: &F, proque: ProQue, quantity: usize) -> Particles {
        let gl_side = GlSide {
            positions: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            velocities: VertexBuffer::empty_dynamic(facade, quantity).unwrap()
        };
        let cl_side = ClSide {
            positions: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                [quantity * 3], gl_side.positions.get_id()).unwrap(),
            velocities: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                [quantity * 3], gl_side.velocities.get_id()).unwrap(),
            proque: proque
        };
        Particles {
            quantity: quantity,
            gl_side: gl_side,
            cl_side: cl_side
        }
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

        let point: &[f32] = &[0.0, 0.0, 0.0];

        self.cl_side.proque.create_kernel("update").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .arg_vec(gravity_point)
            .enq().unwrap();

        self.release_buffers();
    }
}
