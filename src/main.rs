#[macro_use] extern crate glium;
extern crate ocl;
extern crate cgl;

mod particles;
mod point;

use particles::Particles;
use point::Point;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;

use cgl::{CGLGetCurrentContext, CGLGetShareGroup};

fn main() {
    let display = glium::glutin::WindowBuilder::new()
                    .with_dimensions(1024, 768)
                    .with_title(String::from("Hello world"))
                    .build_glium().unwrap();

    let cgl_current_ctx = unsafe { CGLGetCurrentContext() };
    let cgl_share_grp = unsafe { CGLGetShareGroup(cgl_current_ctx) };
    let properties = ContextProperties::new().cgl_sharegroup(cgl_share_grp);

    let device_type = DeviceType::from_bits_truncate(cl_h::CL_DEVICE_TYPE_GPU);
    let devices = Device::list(&Platform::default(), Some(device_type));
    let device = devices.first().expect("No device with specified types found.");

    println!("device: {:?}", device.info(DeviceInfo::Name));

    let context_cl = Context::builder().properties(properties)
                    .devices(DeviceSpecifier::Single(*device))
                    .build().unwrap();

    let mut particles = Particles::new(&display, context_cl, 1_000_000);
    particles.set_background_color(0.17578125, 0.17578125, 0.17578125);
    particles.init_sphere();

    let grav_point = Point::new(0.0, 0.0, 0.0);

    for event in display.wait_events() {
        let mut frame = display.draw();
        particles.update(grav_point);
        particles.draw(&mut frame);

        // println!("{:?}", event);

        match event {
            Event::Closed
            | Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) => break,
            _ => ()
        }
    }
}
