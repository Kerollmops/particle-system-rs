#[macro_use] extern crate glium;
extern crate cgmath;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
extern crate fps_counter;
extern crate hertz;
extern crate easer; // FIXME
use easer::functions::*; // FIXME
mod particles;
mod point;

use std::env;
use glium::{DisplayBuild, Surface};
use glium::glutin::Event;
use glium::glutin::ElementState::Released;
use glium::glutin::VirtualKeyCode::Escape;
use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;
use cgl::{CGLGetCurrentContext, CGLGetShareGroup};
use fps_counter::FPSCounter;
use particles::Particles;
use point::Point;

const GRAY_BACK: (f32, f32, f32, f32) = (0.17578125, 0.17578125, 0.17578125, 1.0);
const MAX_FPS: usize = 60;

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
    particles.init_cube();

    // let mut y = [0.0f32; 11];
    // for i in 0..11 {
    //     y[i] = i as f32 / 10.0;
    // }
    // println!("Before {:?}", &y[..]);
    // y.iter_mut().map(|a| *a = Back::ease_in(*a, 0f32, 1f32, 1f32)).count();
    // println!("After {:?}", &y[..]);

    let grav_point = Point::new(0.0001, 0.0001, 0.0);

    let mut fps_counter = FPSCounter::new();
    loop {
        let ns_at_frame_start = hertz::current_time_ns();

        for event in display.poll_events() {
            // println!("event: {:?}", event);
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { return ; },
                _ => ()
            }
        }

        particles.update(grav_point);
        let mut frame = display.draw();
        frame.clear_color_srgb_and_depth(GRAY_BACK, 1.0);
        particles.draw(&mut frame);

        let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
        display.get_window().unwrap().set_title(&title);
        hertz::sleep_for_constant_rate(MAX_FPS, ns_at_frame_start);
    }
}
