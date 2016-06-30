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
mod animation;

use std::env;
use time::{Duration, PreciseTime};
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::{Escape, Space, C, S};
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

fn resize_window(width: u32, height: u32) {
    println!("resize: {:?}x{:?}", width, height);
}

fn setup_animation(animation: &Animation, particles: &mut Particles) {
    let anim_duration = animation.animation_duration;
    match animation.animation_type {
        AnimationType::Cube => particles.init_cube_animation(anim_duration),
        AnimationType::RandCube => particles.init_rand_cube_animation(anim_duration),
        AnimationType::RandSphere => particles.init_rand_sphere_animation(anim_duration),
    }
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

    let mut animation = Animation {
        animation_type: AnimationType::RandCube,
        animation_duration: Duration::milliseconds(1000),
        animation_start: PreciseTime::now(),
        currently_in_animation: true,
    };
    setup_animation(&animation, &mut particles);

    let camera = Camera::new(&display, width, height);

    let grav_point = Point::new(0.0, 0.0, 0.0);
    let mut update_particles = true;

    let mut fps_counter = FPSCounter::new();
    'game: loop {
        let elaps_time_program = program_start.to(PreciseTime::now());
        let frame_start_time = hertz::current_time_ns();
        for event in display.poll_events() {
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { break 'game; },
                Event::KeyboardInput(Released, _, Some(C)) => {
                    animation_start = PreciseTime::now();
                    in_animation = true;
                    particles.init_rand_cube_animation(anim_duration);
                }
                Event::KeyboardInput(Released, _, Some(S)) => {
                    animation_start = PreciseTime::now();
                    in_animation = true;
                    particles.init_rand_sphere_animation(anim_duration);
                }
                Event::KeyboardInput(Released, _, Some(Space)) => {
                    update_particles = !update_particles;
                },
                _ => ()
            }
        }

        if update_particles == true {
            if in_animation == true {
                let elaps_time_anim = animation_start.to(PreciseTime::now());
                if elaps_time_anim <= anim_duration {
                    particles.update_animation(elaps_time_anim);
                }
                else {
                    particles.update_animation(anim_duration);
                    in_animation = false;
                }
            }
            else {
                particles.update_gravitation(grav_point, elaps_time_program);
            }
        }
        camera.draw(&display, &particles, elaps_time_program);
        let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
        display.get_window().unwrap().set_title(&title);
        hertz::sleep_for_constant_rate(MAX_FPS, frame_start_time);
    }
}
