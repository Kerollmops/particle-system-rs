#[macro_use] extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
#[macro_use] extern crate conrod;
extern crate nalgebra;
extern crate time;
extern crate ocl;
extern crate cgl;
#[macro_use] extern crate colorify;
extern crate fps_counter;
extern crate rustyline;

mod particles;
mod point;
mod camera;
mod animation;

use std::env;
use std::path::Path;
use std::str::FromStr;
use time::{Duration, PreciseTime};
use glium_graphics::{GliumWindow, OpenGL, GlyphCache, Texture, Glium2d};
use piston::window::{Window, WindowSettings, AdvancedWindow};
use piston::event_loop::EventLoop;
use conrod::{Labelable, Positionable, Sizeable, Theme, Widget, Colorable};
use piston::input::*;
use ocl::{Device, Platform, Context, cl_h};
use ocl::core::{ContextProperties, DeviceType, DeviceInfo};
use ocl::builders::DeviceSpecifier;
use cgl::{CGLGetCurrentContext, CGLGetShareGroup};
use fps_counter::FPSCounter;
use particles::{Particles, AnimationFunction, retrieve_quantity, correct_quantity};
use camera::Camera;
use point::Point;
use animation::{AnimationType, Animation};

const MAX_FPS: u64 = 60;

// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (Texture, GlyphCache<GliumWindow>);
type Ui = conrod::Ui<Backend>;

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

    let opengl = OpenGL::V3_2;
    let (width, height) = (1024, 768);
    let title = format!("Particle system in Rust ({} fps)", 30);
    let mut display: GliumWindow = WindowSettings::new(title, [width, height])
                                    .vsync(true)
                                    .opengl(opengl)
                                    .resizable(false) // make it true !!!
                                    .controllers(true) // game controllers ?
                                    .build().unwrap();

    let mut ui = {
        let font_path = Path::new("assets/fonts/NotoSans/NotoSans-Regular.ttf");
        let glyph_cache = GlyphCache::new(&font_path, display.clone()).unwrap();
        let theme = Theme::default();
        Ui::new(glyph_cache, theme)
    };

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
    let camera = Camera::new(&display, display.draw_size());

    let mut grav_point = Point::new(0.0, 0.0, 0.0);
    let mut update_gravitation = false;

    let mut fps_counter = FPSCounter::new();
    let elaps_time_program = program_start.to(PreciseTime::now());

    let mut selected_animation = Some(0);
    let animations = AnimationFunction::all_variants();
    let mut animations_string: Vec<_> = animations.iter().map(|x| String::from_str((*x).into()).unwrap()).collect();

    display.set_ups(MAX_FPS);
    display.set_max_fps(MAX_FPS);
    let mut g2d = Glium2d::new(opengl, &display);
    'game: while let Some(event) = display.next() {
        ui.handle_event(event.clone());
        match event {
            Event::Render(args) => {
                let mut frame = display.draw();
                camera.draw(&mut frame, &display, &particles, elaps_time_program);
                g2d.draw(&mut frame, args.viewport(), |c, g| ui.draw(c, g));
                frame.finish().unwrap();
            }
            Event::Input(Input::Release(Button::Keyboard(Key::Escape))) => {
                break 'game;
            }
            Event::Input(Input::Release(Button::Keyboard(Key::C))) => {
                animation.set_animation(AnimationType::RandCube);
                animation.init_now(&mut particles);
            }
            Event::Input(Input::Release(Button::Keyboard(Key::S))) => {
                animation.set_animation(AnimationType::RandSphere);
                animation.init_now(&mut particles);
            }
            Event::Input(Input::Release(Button::Keyboard(Key::R))) => {
                particles.reset();
                animation.set_animation(AnimationType::RandCube);
                animation.init_now(&mut particles);
                particles.update_animation(animation.duration());
                update_gravitation = false;
            }
            Event::Input(Input::Release(Button::Keyboard(Key::Q))) => {
                if animation.currently_in_animation() == false {
                    particles.change_animation_function(AnimationFunction::QuadEaseInOut);
                }
            }
            Event::Input(Input::Release(Button::Keyboard(Key::E))) => {
                if animation.currently_in_animation() == false {
                    particles.change_animation_function(AnimationFunction::ElasticEaseOut);
                }
            }
            Event::Input(Input::Release(Button::Keyboard(Key::Space))) => {
                update_gravitation = !update_gravitation;
            }
            Event::Update(_) => {
                ui.set_widgets(|ref mut ui| {
                    widget_ids!(CANVAS, COUNTER, DROPDOWN, SLIDER);

                    // Create a background canvas upon which we'll place the button.
                    // conrod::Canvas::new()
                    //     .pad(40.0)
                    //     .color(conrod::color::TRANSPARENT)
                    //     .set(CANVAS, ui);

                    // Draw the button and increment `count` if pressed.
                    conrod::Toggle::new(update_gravitation)
                        // .middle_of(CANVAS)
                        .top_left_with_margin(5.0)
                        .w_h(80.0, 35.0)
                        .color(conrod::color::RED)
                        .label("play/pause") // FIXME change this text
                        .small_font(&ui)
                        .react(|value| update_gravitation = value)
                        .set(COUNTER, ui);

                    conrod::DropDownList::new(&mut animations_string, &mut selected_animation)
                        .down_from(COUNTER, 10.0)
                        .w_h(130.0, 35.0)
                        .max_visible_items(6)
                        .react(|selected_idx: &mut Option<usize>, new_idx, _: &str| {
                            *selected_idx = Some(new_idx);
                            particles.change_animation_function(*animations.get(new_idx).unwrap());
                        })
                        .small_font(&ui)
                        .set(DROPDOWN, ui);

                    conrod::Slider::new(0.5, 0.0, 1.0)
                        .down_from(DROPDOWN, 10.0)
                        .react(|value| println!("value: {:?}", value))
                        .set(SLIDER, ui);
                });
                if animation.currently_in_animation() == true {
                    animation.update(&mut particles);
                }
                else if update_gravitation == true {
                    particles.update_gravitation(grav_point, elaps_time_program);
                }
                let title = format!("Particle system in Rust ({} fps)", fps_counter.tick());
                display.set_title(title);
            },
            _ => ()
        }
    }
}
