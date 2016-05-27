#[macro_use] extern crate glium;
extern crate ocl;

use glium::{DisplayBuild, Surface, VertexBuffer, IndexBuffer, Program, GlObject};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

const VERTEX_SRC: &'static str = include_str!("shaders/default.vert");
const FRAGMENT_SRC: &'static str = include_str!("shaders/default.frag");

use ocl::{util, core, ProQue, Buffer, Device, Platform, Queue, Context, Program as ProgramCl};
use ocl::core::{ContextProperties, DeviceType};
use ocl::builders::DeviceSpecifier;
use ocl::cl_h::CL_DEVICE_TYPE_GPU;

const KERNEL_SRC: &'static str = include_str!("kernels/test.cl");

// Number of results to print out:
const RESULTS_TO_PRINT: usize = 20;

// Our arbitrary data set size and coefficent:
const DATA_SET_SIZE: usize = 2 << 20;
const COEFF: f32 = 5432.1;

#[derive(Copy, Clone)]
struct Particle {
    position: [f32; 2],
    color: [f32; 3], // to delete
}

implement_vertex!(Particle, position, color);

fn main() {
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(String::from("Hello world"))
        .build_glium()
        .unwrap();

    let vertex_buffer = VertexBuffer::dynamic(&display,
            &[
                Particle { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Particle { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Particle { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
            ]).unwrap();
    // let indices = NoIndices(PrimitiveType::Points);
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
    let program = Program::from_source(&display, VERTEX_SRC, FRAGMENT_SRC, None).unwrap();
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    let properties = ContextProperties::new().gl_context();

    let context_cl = Context::new(Some(properties), Some(DeviceSpecifier::First),
                    None, None).unwrap();

    // println!("get_id: {:?}", vertex_buffer.get_id());
    // Create a big ball of OpenCL-ness (see ProQue and ProQueBuilder docs for info):
    // let ocl_pq = ProQue::builder()
    //     .src(KERNEL_SRC)
    //     .dims([DATA_SET_SIZE]) // don't understand
    //     .build().expect("Build ProQue");

    let device_cl = Device::first(*Platform::list().first().unwrap());
    let queue_cl = Queue::new(&context_cl, device_cl).unwrap();

    let program_cl = ProgramCl::builder().src(KERNEL_SRC).build(&context_cl).unwrap();

    let pq_cl = ProQue::new(context_cl, queue_cl, program_cl, Some([DATA_SET_SIZE]));

    // Create a source buffer and initialize it with random floats between 0.0
    // and 20.0 using a temporary init vector, `vec_source`:
    // let vec_source = util::scrambled_vec((0.0, 20.0), ocl_pq.dims().to_len());
    // let vertex_buffer_cl = Buffer::new(ocl_pq.queue(), Some(core::MEM_READ_WRITE |
    //     core::MEM_COPY_HOST_PTR), ocl_pq.dims().clone(), Some(&vec_source)).unwrap();


    let vertex_buffer_cl = Buffer::new().unwrap();

    // create_from_gl_buffer(
    //         context: &Context,
    //         gl_object: cl_GLuint,
    //         flags: MemFlags
    //     ) -> OclResult<Mem>

    // ContextProperty::CglSharegroupKhr

    // Create a kernel with arguments corresponding to those in the kernel:
    let kern = pq_cl.create_kernel("add_to_each").unwrap()
        .arg_scl(0.2)
        .arg_buf(&vertex_buffer_cl);

    println!("Kernel global work size: {:?}", kern.get_gws());

    // Enqueue kernel:
    kern.enq().unwrap();

    // Read results from the device into result_buffer's local vector:
    // result_buffer.read(&mut vec_result).enq().unwrap();

    for event in display.wait_events() {
        let mut frame = display.draw();
        frame.clear_color(0.17, 0.17, 0.17, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();

        // println!("{:?}", event);

        match event {
            Event::Closed
            | Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => break,
            _ => ()
        }
    }
}
