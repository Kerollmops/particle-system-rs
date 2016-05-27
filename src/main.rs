#[macro_use] extern crate glium;
extern crate ocl;

use glium::{DisplayBuild, Surface, VertexBuffer, IndexBuffer, Program, GlObject};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

const VERTEX_SRC: &'static str = include_str!("shaders/default.vert");
const FRAGMENT_SRC: &'static str = include_str!("shaders/default.frag");

use ocl::{util, core, ProQue, Buffer};

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

    // println!("get_id: {:?}", vertex_buffer.get_id());
    // Create a big ball of OpenCL-ness (see ProQue and ProQueBuilder docs for info):
    let ocl_pq = ProQue::builder()
        .src(KERNEL_SRC)
        .dims([DATA_SET_SIZE])
        .build().expect("Build ProQue");

    // Create a source buffer and initialize it with random floats between 0.0
    // and 20.0 using a temporary init vector, `vec_source`:
    let vec_source = util::scrambled_vec((0.0, 20.0), ocl_pq.dims().to_len());
    let source_buffer = Buffer::new(ocl_pq.queue(), Some(core::MEM_READ_WRITE |
        core::MEM_COPY_HOST_PTR), ocl_pq.dims().clone(), Some(&vec_source)).unwrap();

    // Create another empty buffer and vector for results:
    let mut vec_result = vec![0.0f32; DATA_SET_SIZE];
    let result_buffer: Buffer<f32> = ocl_pq.create_buffer().unwrap();

    // Create a kernel with arguments corresponding to those in the kernel:
    let kern = ocl_pq.create_kernel("multiply_by_scalar").unwrap()
        .arg_scl(COEFF)
        .arg_buf(&source_buffer)
        .arg_buf(&result_buffer);

    println!("Kernel global work size: {:?}", kern.get_gws());

    // Enqueue kernel:
    kern.enq().unwrap();

    // Read results from the device into result_buffer's local vector:
    result_buffer.read(&mut vec_result).enq().unwrap();

    // Check results and print the first 20:
    for idx in 0..DATA_SET_SIZE {
        if idx < RESULTS_TO_PRINT {
            println!("source[{idx}]: {:.03}, \t coeff: {}, \tresult[{idx}]: {}",
            vec_source[idx], COEFF, vec_result[idx], idx = idx);
        }
        assert_eq!(vec_source[idx] * COEFF, vec_result[idx]);
    }

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
