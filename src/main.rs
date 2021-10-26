use std::{time::{Duration, Instant}};
#[allow(unused_imports)]
use rand::Rng;

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::{Event, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

mod world;
use crate::world::*;

const SCALE: u32 = 10;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 50;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = WindowBuilder::new()
        .with_title("Sandbox")
        .with_inner_size(PhysicalSize::new(SCALE*(WIDTH + 3), SCALE*HEIGHT))
        .with_min_inner_size(PhysicalSize::new(200, 200))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(SCALE*(WIDTH + 3), SCALE*HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH + 3, HEIGHT, surface_texture).unwrap();
    init_pixels(pixels.get_frame());

    let buff_a = vec![Particle::Void; (WIDTH*HEIGHT) as usize].into_boxed_slice();
    let buff_b = vec![Particle::Void; (WIDTH*HEIGHT) as usize].into_boxed_slice();
    let mut world = World::new(WIDTH as usize, HEIGHT as usize, buff_a, buff_b);

    let mut last = Instant::now();
    //let mut rng = rand::thread_rng();

    let mut selected_particle = Particle::Sand;
    let bindings = [//temp
        Binding { key: VirtualKeyCode::A, particle: Particle::Sand },
        Binding { key: VirtualKeyCode::Z, particle: Particle::Stone },
        Binding { key: VirtualKeyCode::E, particle: Particle::Water }
    ];
    
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
            }

            for binding in bindings.iter() {
                if input.key_pressed(binding.key) {
                    selected_particle = binding.particle;
                }
            }

            if input.mouse_held(0) {
                match input.mouse() {
                    Some((x, y)) => world.spawn((x/SCALE as f32).floor() as usize, (y/SCALE as f32).floor() as usize, selected_particle),
                    None => ()
                }
            }
            if input.mouse_pressed(1) {
                match input.mouse() {
                    Some((x, y)) => world.spawn((x/SCALE as f32).floor() as usize, (y/SCALE as f32).floor() as usize, selected_particle),
                    None => ()
                }
            }

            let now = Instant::now();
            if now.duration_since(last) >= Duration::from_millis(20) {
                last = now;
                world.update();
            }

            window.request_redraw();
        }

        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame(), selected_particle);
            pixels.render().unwrap();
        }
    })
}

fn init_pixels(frame: &mut [u8]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = 4*(x + y*WIDTH) as usize;
            frame[index + 0] = 0;
            frame[index + 1] = 0;
            frame[index + 2] = 0;
        }
    }
}

struct Binding {
    key: VirtualKeyCode,
    particle: Particle
}