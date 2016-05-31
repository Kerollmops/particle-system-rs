#[macro_use] extern crate glium;
extern crate nalgebra;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
extern crate fps_counter;
extern crate hertz;
mod particles;
mod point;
mod camera;

use std::env;
use glium::{DisplayBuild, Surface};
use glium::glutin::Event;
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::Escape;
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::Facade;
use glium::Frame;
use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;
use cgl::{CGLGetCurrentContext, CGLGetShareGroup};
use fps_counter::FPSCounter;
use particles::Particles;
use camera::Camera;
use point::Point;

const BACKGROUND: (f32, f32, f32, f32) = (0.17578, 0.17578, 0.17578, 1.0);
// const BACKGROUND: (f32, f32, f32, f32) = (0.02343, 0.02343, 0.02343, 1.0);
// const BACKGROUND: (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0);
const MAX_FPS: usize = 60;

// FIXME delete
enum AnimationType {
    RandCube,
    Cube,
    RandSphere
}

fn draw(frame: &mut Frame, camera: &Camera, particles: &Particles) {
    let indices = NoIndices(PrimitiveType::Points);
    let uniforms = uniform!{ matrix: *camera.matrix().as_ref() };
    frame.draw(particles.positions(), &indices, particles.program(),
        &uniforms, &Default::default()).unwrap();
    frame.set_finish().unwrap();
}

fn main() {
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(1024, 768)
                    .with_title(format!("Particle system in Rust ({} fps)", 30))
                    .build_glium().unwrap();

    let device_type = DeviceType::from_bits_truncate(cl_h::CL_DEVICE_TYPE_GPU);
    let devices = Device::list(&Platform::default(), Some(device_type));
    let device = devices.first().expect("No device with specified types found.");

    println!("Device used: {:?}", device.info(DeviceInfo::Name));

    let cgl_current_ctx = unsafe { CGLGetCurrentContext() };
    let cgl_share_grp = unsafe { CGLGetShareGroup(cgl_current_ctx) };
    let properties = ContextProperties::new().cgl_sharegroup(cgl_share_grp);
    let context_cl = Context::builder().properties(properties)
                        .devices(DeviceSpecifier::Single(*device))
                        .build().unwrap();

    let quantity: usize = env::args().nth(1)
                            .unwrap_or(String::from("1000000")).parse()
                            .unwrap_or_else(|err| { printlnc!(red: "{}", err); 1_000_000 });

    let mut particles = match Particles::new(&display, context_cl, quantity) {
        Ok(particles) => particles,
        Err(err) => { printlnc!(red: "{}", err); return ; }
    };
    println!("{} particles will be emitted!", quantity);

    // let start_animation = use std::time::Duration;
    let mut global_timer = 0.0_f32; // FIXME use duration
    let mut anim_timer = 0.0_f32; // FIXME use duration
    let anim_duration = 0.7_f32;
    let mut anim_type = AnimationType::RandSphere;
    match anim_type {
        AnimationType::Cube => particles.init_cube_animation(anim_duration),
        AnimationType::RandCube => particles.init_rand_cube_animation(anim_duration),
        AnimationType::RandSphere => particles.init_rand_sphere_animation(anim_duration),
    }

    let (width, height) = (1024.0, 768.0);
    let mut camera = Camera::new(width / height);
    let grav_point = Point::new(0.0, 0.0, 0.0);

    let mut fps_counter = FPSCounter::new();
    loop {
        let frame_start_time = hertz::current_time_ns();
        for event in display.poll_events() {
            // println!("event: {:?}", event);
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { return ; },
                _ => ()
            }
        }

        global_timer += 0.01;
        if anim_timer <= anim_duration {
            particles.update_animation(anim_timer);
            anim_timer += 0.01;
        }
        else {
            particles.update_gravitation(grav_point, global_timer);
        }

        // else {
        //     anim_type = match anim_type {
        //         AnimationType::Cube => {
        //             particles.init_rand_sphere_animation(anim_duration);
        //             AnimationType::RandSphere
        //         },
        //         AnimationType::RandSphere => {
        //             // particles.init_rand_cube_animation(anim_duration);
        //             // AnimationType::RandCube
        //             particles.init_cube_animation(anim_duration);
        //             AnimationType::Cube
        //         },
        //         AnimationType::RandCube => {
        //             particles.init_cube_animation(anim_duration);
        //             AnimationType::Cube
        //         }
        //     };
        //     anim_timer = 0.00;
        // }

        let mut frame = display.draw();
        frame.clear_color_srgb_and_depth(BACKGROUND, 1.0);
        draw(&mut frame, &camera, &particles);

        let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
        display.get_window().unwrap().set_title(&title);
        hertz::sleep_for_constant_rate(MAX_FPS, frame_start_time);
    }
}
