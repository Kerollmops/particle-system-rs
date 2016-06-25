#[macro_use] extern crate glium;
extern crate nalgebra;
extern crate time;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
extern crate fps_counter;
extern crate hertz;
mod particles;
mod point;
mod camera;

use std::env;
use time::{Duration, PreciseTime};
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::{Escape, Space};
use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;
use cgl::{CGLGetCurrentContext, CGLGetShareGroup};
use fps_counter::FPSCounter;
use particles::Particles;
use camera::Camera;
use point::Point;

const MAX_FPS: usize = 60;
const WARP_SIZE: usize = 32;

// FIXME delete
enum AnimationType {
    RandCube,
    Cube,
    RandSphere
}

fn resize_window(width: u32, height: u32) {
    println!("resize: {:?}x{:?}", width, height);
}

fn main() {
    let (width, height) = (1024.0, 768.0);
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(width as u32, height as u32)
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
    let quantity = ((quantity / WARP_SIZE) + 1) * WARP_SIZE;

    let mut particles = match Particles::new(&display, context_cl, quantity) {
        Ok(particles) => particles,
        Err(err) => { printlnc!(red: "{}", err); return ; }
    };
    println!("{} particles will be emitted!", quantity);

    let program_start = PreciseTime::now();
    let mut animation_start = PreciseTime::now();
    let anim_duration = Duration::milliseconds(1700);
    let mut anim_type = AnimationType::RandCube;
    match anim_type {
        AnimationType::Cube => particles.init_cube_animation(anim_duration),
        AnimationType::RandCube => particles.init_rand_cube_animation(anim_duration),
        AnimationType::RandSphere => particles.init_rand_sphere_animation(anim_duration),
    }

    let camera = Camera::new(&display, width, height);

    let grav_point = Point::new(0.0, 0.0, 0.0);
    let mut update_particles = true;

    let mut fps_counter = FPSCounter::new();
    'game: loop {
        let frame_start_time = hertz::current_time_ns();
        for event in display.poll_events() {
            // println!("event: {:?}", event);
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { break 'game; },
                Event::KeyboardInput(Released, _, Some(Space)) => {
                    update_particles = !update_particles;
                },
                _ => ()
            }
        }

        let elaps_time_program = program_start.to(PreciseTime::now());

        if update_particles == true {
            let elaps_time_anim = animation_start.to(PreciseTime::now());
            // println!("elaps_time_anim: {:?}", elaps_time_anim.num_milliseconds() as f32);
            // println!("anim_duration: {:?}", anim_duration.num_milliseconds() as f32);
            if elaps_time_anim <= anim_duration {
                // println!("update!");
                particles.update_animation(elaps_time_anim);
            }
            else {
                anim_type = match anim_type {
                    AnimationType::Cube => {
                        animation_start = PreciseTime::now();
                        particles.init_rand_sphere_animation(anim_duration);
                        AnimationType::RandSphere
                    },
                    AnimationType::RandSphere => {
                        animation_start = PreciseTime::now();
                        particles.init_rand_cube_animation(anim_duration);
                        AnimationType::RandCube
                        // particles.init_cube_animation(anim_duration);
                        // AnimationType::Cube
                    },
                    AnimationType::RandCube => {
                        animation_start = PreciseTime::now();
                        particles.init_cube_animation(anim_duration);
                        AnimationType::Cube
                    }
                };
            }
            // else {
            //     particles.update_gravitation(grav_point, elaps_time_program);
            // }
        }

        camera.draw(&display, &particles, elaps_time_program);

        // println!("sin(time) = {:?}", (global_timer).sin());

        let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
        display.get_window().unwrap().set_title(&title);
        hertz::sleep_for_constant_rate(MAX_FPS, frame_start_time);
    }
}
