#[macro_use] extern crate glium;
extern crate nalgebra;
extern crate time;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
extern crate fps_counter;
extern crate hertz;
extern crate rustyline;

mod particles;
mod point;
mod camera;
mod animation;

use std::env;
use time::{Duration, PreciseTime};
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::{Escape, Space, C, S, E, Q, R};
use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;
use cgl::{CGLGetCurrentContext, CGLGetShareGroup};
use fps_counter::FPSCounter;
use particles::{Particles, AnimationFunction, retrieve_quantity, correct_quantity};
use camera::Camera;
use point::Point;
use animation::{AnimationType, Animation};

const MAX_FPS: usize = 60;

fn resize_window(width: u32, height: u32) {
    println!("resize: {:?}x{:?}", width, height);
}

fn contains((width, height): (i32, i32), (x, y): (i32, i32)) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn main() {
    let quantity = retrieve_quantity(env::args().nth(1));
    let quantity = match correct_quantity(quantity) {
        Err(err) => return printlnc!(red: "{}", err),
        Ok(x) => x
    };

    let (width, height) = (1024.0, 768.0);
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(width as u32, height as u32)
                    .with_title(format!("Particle system in Rust ({} fps)", 30))
                    .with_vsync()
                    .build_glium().unwrap();

    let device_type = DeviceType::from_bits_truncate(cl_h::CL_DEVICE_TYPE_GPU);
    let devices = Device::list(&Platform::default(), Some(device_type));
    let device = devices.first().expect("No device with specified types found.");

    println!("Device used: {:?}", device.info(DeviceInfo::Name));

    let cgl_current_context = unsafe { CGLGetCurrentContext() };
    let cgl_share_group = unsafe { CGLGetShareGroup(cgl_current_context) };
    let properties = ContextProperties::new().cgl_sharegroup(cgl_share_group);
    let context_cl = Context::builder().properties(properties)
                        .devices(DeviceSpecifier::Single(*device))
                        .build().unwrap();

    let mut particles = Particles::new(&display, context_cl, quantity);
    println!("{} particles will be emitted!", quantity);

    let program_start = PreciseTime::now();
    let mut animation = Animation::new(Duration::milliseconds(1000));
    animation.init_now(&mut particles);
    let camera = Camera::new(&display, width, height);

    let mut grav_point = Point::new(0.0, 0.0, 0.0);
    let mut update_gravitation = true;

    let mut fps_counter = FPSCounter::new();
    'game: loop {
        let elaps_time_program = program_start.to(PreciseTime::now());
        let frame_start_time = hertz::current_time_ns();

        for event in display.poll_events() {
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { break 'game; },
                Event::MouseMoved(x, y) => {
                    if contains((width as i32, height as i32), (x, y)) {
                        // grav_point = Point::new(x as f32, y as f32, 0.0);
                    }
                    // println!("mouse moved {}:{} [{}]", x, y, is_in);
                }
                Event::KeyboardInput(Released, _, Some(C)) => {
                    animation.set_animation(AnimationType::RandCube);
                    animation.init_now(&mut particles);
                }
                Event::KeyboardInput(Released, _, Some(S)) => {
                    animation.set_animation(AnimationType::RandSphere);
                    animation.init_now(&mut particles);
                }
                Event::KeyboardInput(Released, _, Some(R)) => {
                    particles.reset();
                    animation.set_animation(AnimationType::RandCube);
                    animation.init_now(&mut particles);
                    particles.update_animation(animation.duration());
                    update_gravitation = false;
                }
                Event::KeyboardInput(Released, _, Some(E)) => {
                    if animation.currently_in_animation() == false {
                        particles.change_animation_function(AnimationFunction::ElasticEaseOut);
                    }
                }
                Event::KeyboardInput(Released, _, Some(Q)) => {
                    if animation.currently_in_animation() == false {
                        particles.change_animation_function(AnimationFunction::QuadEaseInOut);
                    }
                }
                Event::KeyboardInput(Released, _, Some(Space)) => {
                    update_gravitation = !update_gravitation;
                },
                _ => ()
            }
        }

        if animation.currently_in_animation() == true {
            animation.update(&mut particles);
        }
        else if update_gravitation == true {
            particles.update_gravitation(grav_point, elaps_time_program);
        }

        camera.draw(&display, &particles, elaps_time_program);
        let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
        display.get_window().unwrap().set_title(&title);
        hertz::sleep_for_constant_rate(MAX_FPS, frame_start_time);
    }
}
