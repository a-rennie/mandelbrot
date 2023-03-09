use angular_units::Deg;
use num::complex::ComplexFloat;
use num::Complex;
use prisma::FromColor;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;
use prisma::encoding::{EncodableColor, TranscodableColor, SrgbEncoding};

fn mandelbrot(coord: Complex<f64>, max_iter: u64) -> u64 {
    let mut iteration = 0;
    let mut z = Complex::new(0.0, 0.0);
    while z.abs() <= 4.0 && iteration < max_iter {
        z = z * z + coord;
        iteration += 1
    }
    iteration
}

const ZOOM: f64 = 1.0 / 4.5e-14;
const MAX_ITER: u64 = 1000;
const X_OFFSET: f64 = -0.7336438924199521;
const Y_OFFSET: f64 = 0.2455211406714035;
const MIN_X: f64 = -2.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = -1.0;
const MAX_Y: f64 = 1.0;

const SIZE: f64 = 500.0;

const WIDTH: i32 = (SIZE * (MAX_X - MIN_X)) as i32;
const HEIGHT: i32 = (SIZE * (MAX_Y - MIN_Y)) as i32;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(127, 127, 127));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x0 = (((((MAX_X - MIN_X) / WIDTH as f64) * i as f64) + MIN_X) / ZOOM) + X_OFFSET;
            let y0 = (((((MAX_Y - MIN_Y) / HEIGHT as f64) * j as f64) + MIN_Y) / ZOOM) + Y_OFFSET;
            let scaledcoords = Complex::new(x0, y0);
            let iteration = mandelbrot(scaledcoords, MAX_ITER);
            //let iteration = format!("000000{:X}", iteration);
            //println!("{iteration}");
            let colour = prisma::Hsv::new(
                Deg(180.0 * (iteration as f64 / MAX_ITER as f64)),
                1.0,
                if iteration < MAX_ITER { 1.0 } else { 0.0 },
            );
            let colour = prisma::Rgb::from_color(&colour);
            let colour = colour.encode(SrgbEncoding);
            let red: f64 = colour.red() * 255.0;
            let blue: f64 = colour.blue() * 255.0;
            let green: f64 = colour.green() * 255.0;

            let red = red.round() as u8;
            let blue = blue.round() as u8;
            let green = green.round() as u8;
            /*
            let iteration = &iteration[iteration.len() - 6 .. iteration.len()];

            let rgbvals = decode_hex(iteration).unwrap();
            let red = rgbvals[0];
            let green = rgbvals[1];
            let blue = rgbvals[2];
            */
            canvas.set_draw_color(Color::RGB(red, green, blue));
            canvas.draw_point(Point::new(i, j));
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            //canvas.present();
            //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        canvas.present();
        //  ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    //canvas.present();
    'keepopen: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'keepopen,
                _ => {}
            }
        }
    }
}
