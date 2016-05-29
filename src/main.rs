#[macro_use] extern crate glium;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
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
use particles::Particles;
use point::Point;

fn main() {
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(1024, 768)
                    .with_title(String::from("Particle system in Rust"))
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
        Err(err) => { printlnc!(red: "{}", err); return ; },
    };
    println!("{} particles will be emitted!", quantity);
    particles.init_cube();

    let grav_point = Point::new(10.0, 10.0, 0.0);

    // let title = ; // set_title(&self, title: &str)

    loop {
        for event in display.poll_events() {
            // println!("event: {:?}", event);
            match event {
                Event::Closed
                | Event::KeyboardInput(Released, _, Some(Escape)) => { return ; },
                _ => ()
            }
        }

        // println!("caca");

        particles.update(grav_point);
        let mut frame = display.draw();
        frame.clear_color_srgb_and_depth((0.17578125, 0.17578125, 0.17578125, 1.0), 1.0);
        particles.draw(&mut frame);
    }
}
