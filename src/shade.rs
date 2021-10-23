use std::ops::Sub;

use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, event::{Event, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

const SCALE: u32 = 2;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = WindowBuilder::new()
        .with_title("Sandbox")
        .with_inner_size(PhysicalSize::new(SCALE*256, SCALE*256))
        .with_min_inner_size(PhysicalSize::new(200, 200))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(SCALE*256, SCALE*256, &window);
    let mut pixels = Pixels::new(256, 256, surface_texture).unwrap();
    let mut v = [(255, true), (255, true), (255, true)];

    init_pixels(pixels.get_frame());
    
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame(), &mut v);
            pixels.render().unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
            }

            window.request_redraw();
        }
    })
}

fn init_pixels(frame: &mut [u8]) {
    for y in 0..256 {
        for x in 0..256 {
            let index = 4*(x + y*256) as usize;
            frame[index + 0] = x as u8;
            frame[index + 1] = y as u8;
            frame[index + 2] = std::cmp::max(255 - x, 255 - y) as u8;
        }
    }
}

fn draw(frame: &mut [u8], v: &mut [(u32, bool)]) {
    for y in 0..256 {
        for x in 0..256 {
            let i = 4*(x + y*256) as usize;

            frame[i + 0] = 255 - diff(x, v[0].0) as u8;
            frame[i + 1] = 255 - diff(y, v[1].0) as u8;
            frame[i + 2] = 255 - diff(std::cmp::max(255 - x, 255 - y), v[2].0) as u8;
        }
    }

    for chan in v {
        if chan.0 == 0 { chan.1 = true }
        else if chan.0 == 255 { chan.1 = false }

        if chan.1 { chan.0 += 1 }
        else { chan.0 -= 1 }
    }
}

fn diff<T: Sub<Output = T> + Ord>(a: T, b: T) -> T {
    if a > b { a - b }
    else { b - a }
}