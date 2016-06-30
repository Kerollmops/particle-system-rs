use std::result::Result;
use std::convert::Into;
use glium::{VertexBuffer, GlObject, Program};
use glium::backend::Facade;
use ocl::{Buffer, ProQue, Context, Program as ClProgram};
use ocl::builders::BuildOpt;
use ocl::aliases::ClFloat3;
use ocl::core::MEM_READ_WRITE;
use animation::{Animation, AnimationType};
use time::Duration;
use point::Point;

const DEFAULT_QUANTITY: usize = 1_000_000;
const MAX_QUANTITY: usize = 3_000_000;
const WARP_SIZE: usize = 32;
const PARTICLES_CL: &'static str = include_str!("kernels/particles.cl");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationFunction {
    SineEaseInOut,
    BackEaseInOut,
    QuadEaseInOut,
    BackEaseOut,
    ElasticEaseOut,
}

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
    velocities: VertexBuffer<Velocity>
}

struct AnimationBuffers {
    from: Buffer<ClFloat3>,
    to: Buffer<ClFloat3>,
    duration: f32
}

struct ClSide {
    positions: Buffer<ClFloat3>,
    velocities: Buffer<ClFloat3>,
    animation: AnimationBuffers,
    anim_func: AnimationFunction,
    context: Context,
    proque: ProQue
}

pub struct Particles {
    quantity: usize,
    gl_side: GlSide,
    cl_side: ClSide
}

pub fn retrieve_quantity(first_arg: Option<String>) -> usize {
    if let Some(str_quantity) = first_arg {
        if let Ok(quantity) = str_quantity.parse() {
            return quantity;
        }
    }
    DEFAULT_QUANTITY
}

pub fn correct_quantity(quantity: usize) -> Result<usize, &'static str> {
    if quantity == 0 {
        Err("Cannot emit zero particles.")
    }
    else if quantity > MAX_QUANTITY {
        Err("Cannot emit more than 3 millions particles.")
    }
    else {
        Ok(((quantity / WARP_SIZE) + 1) * WARP_SIZE)
    }
}

fn compute_proque(context: Context, build_option: BuildOpt, quantity: usize) -> ProQue {
    let prog_bldr = ClProgram::builder().bo(build_option).src(PARTICLES_CL);
    let device = context.devices().first().unwrap().clone();
    ProQue::builder().context(context).prog_bldr(prog_bldr).device(device)
            .dims([quantity]).build().unwrap()
}

fn create_cl_side_animation(anim_func: AnimationFunction,
                            context: Context,
                            gl_side: &GlSide,
                            quantity: usize) -> ClSide {

    let anim_func_name: &'static str = anim_func.into();
    let easing_animation = BuildOpt::CmplrDefine {
        ident: "EASING_ANIMATION".into(),
        val: anim_func_name.into(),
    };
    let proque = compute_proque(context.clone(), easing_animation, quantity);
    ClSide::new(proque, context, &gl_side, quantity, anim_func)
}

impl Into<&'static str> for AnimationFunction {
    fn into(self) -> &'static str {
        match self {
            AnimationFunction::SineEaseInOut => "sine_ease_in_out",
            AnimationFunction::BackEaseInOut => "back_ease_in_out",
            AnimationFunction::QuadEaseInOut => "quad_ease_in_out",
            AnimationFunction::BackEaseOut => "back_ease_out",
            AnimationFunction::ElasticEaseOut => "elastic_ease_out",
        }
    }
}

impl Into<&'static str> for AnimationType {
    fn into(self) -> &'static str {
        match self {
            AnimationType::RandCube => "init_rand_cube_animation",
            AnimationType::Cube => "init_cube_animation",
            AnimationType::RandSphere => "init_rand_sphere_animation",
        }
    }
}

impl ClSide {
    pub fn new(proque: ProQue, context: Context, gl_side: &GlSide,
               quantity: usize, anim_func: AnimationFunction) -> ClSide {
        ClSide {
            positions: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                        [quantity], gl_side.positions.get_id()).unwrap(),
            velocities: Buffer::from_gl_buffer(&proque, Some(MEM_READ_WRITE),
                        [quantity], gl_side.velocities.get_id()).unwrap(),
            animation: AnimationBuffers {
                from: Buffer::new(&proque, Some(MEM_READ_WRITE), [quantity], None).unwrap(),
                to: Buffer::new(&proque, Some(MEM_READ_WRITE), [quantity], None).unwrap(),
                duration: Default::default(),
            },
            anim_func: anim_func,
            context: context,
            proque: proque
        }
    }
}

impl Particles {
    pub fn new<F: Facade>(facade: &F, context: Context, quantity: usize) -> Particles {
        let gl_side = GlSide {
            positions: VertexBuffer::empty_dynamic(facade, quantity).unwrap(),
            velocities: VertexBuffer::empty_dynamic(facade, quantity).unwrap()
        };
        let cl_side = create_cl_side_animation(AnimationFunction::QuadEaseInOut,
                        context, &gl_side, quantity);
        Particles {
            quantity: quantity,
            gl_side: gl_side,
            cl_side: cl_side
        }
    }

    pub fn change_animation_function(&mut self, anim_func: AnimationFunction) {
        if self.cl_side.anim_func != anim_func {
            self.cl_side.anim_func = anim_func;
            self.cl_side = create_cl_side_animation(self.cl_side.anim_func,
                            self.cl_side.context.clone(),
                            &self.gl_side,
                            self.quantity);
        }
    }

    pub fn animation_function(&self) -> AnimationFunction {
        self.cl_side.anim_func
    }

    pub fn reset(&mut self) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel("reset").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn init_animation(&mut self, duration: Duration, anim_type: AnimationType) {
        let anim_func = anim_type.into();
        self.cl_side.animation.duration = duration.num_milliseconds() as f32;

        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel(anim_func).unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.velocities)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn update_animation(&mut self, time: Duration) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel("update_animation").unwrap()
            .arg_buf(&self.cl_side.animation.from)
            .arg_buf(&self.cl_side.animation.to)
            .arg_buf(&self.cl_side.positions)
            .arg_scl(time.num_milliseconds() as f32)
            .arg_scl(self.cl_side.animation.duration)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
    }

    pub fn update_gravitation(&mut self, gravity_point: Point, time: Duration) {
        self.cl_side.positions.cmd().gl_acquire().enq().unwrap();
        self.cl_side.velocities.cmd().gl_acquire().enq().unwrap();

        self.cl_side.proque.create_kernel("update_gravitation").unwrap()
            .arg_buf(&self.cl_side.positions)
            .arg_buf(&self.cl_side.velocities)
            .arg_vec(gravity_point)
            .arg_scl(time.num_milliseconds() as f32)
            .enq().unwrap();

        self.cl_side.positions.cmd().gl_release().enq().unwrap();
        self.cl_side.velocities.cmd().gl_release().enq().unwrap();
    }

    pub fn positions(&self) -> &VertexBuffer<Position> {
        &self.gl_side.positions
    }
}
