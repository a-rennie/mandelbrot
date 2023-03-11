use colorsys::{Hsl, Rgb};
use image::{Rgb as image_rgb, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashMap;

fn mandelbrot(coord: Complex<f64>, max_iter: u64) -> u64 {
    let mut iteration = 0;
    let mut z = Complex::new(0.0, 0.0);
    while z.abs() <= 4.0 && iteration < max_iter {
        z = z * z + coord;
        iteration += 1
    }
    iteration
}

const ZOOM: f64 = 1.0 / 2.076078511869575e-7; // 1.0 for full set;
const MAX_ITER: u64 = 10000;
const X_OFFSET: f64 = 0.4325684481884891; //0.0 for full set;
const Y_OFFSET: f64 = 0.22611198415267986; //o.0 for full set;
const MIN_X: f64 = -2.0; //-2.0 recommended
const MAX_X: f64 = 1.0; //1.0 recommended
const MIN_Y: f64 = -1.2; //-1.2 recommended
const MAX_Y: f64 = 1.2; // 1.2 recommended

const SIZE: f64 = 5000.0; //500 is recommended

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

    let mut iteration_counts = HashMap::new();
    let mut iter_per_pixel: [u32; MAX_ITER as usize + 1] = [0; MAX_ITER as usize + 1];

    'running: for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x0 = (((((MAX_X - MIN_X) / WIDTH as f64) * i as f64) + MIN_X) / ZOOM) + X_OFFSET;
            let y0 = (((((MAX_Y - MIN_Y) / HEIGHT as f64) * j as f64) + MIN_Y) / ZOOM) + Y_OFFSET;
            let scaledcoords = Complex::new(x0, y0);
            let iteration = mandelbrot(scaledcoords, MAX_ITER);
            if iteration < MAX_ITER {
                iter_per_pixel[iteration as usize] += 1;
            }
            iteration_counts.insert((i, j), iteration);
            canvas.set_draw_color(Color::RGB(
                ((iteration / MAX_ITER) * 255) as u8,
                ((iteration / MAX_ITER) * 255) as u8,
                ((iteration / MAX_ITER) * 255) as u8,
            ));
            canvas
                .draw_point(Point::new(i, j))
                .expect("failed to draw pixel");
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
    let total: u32 = iter_per_pixel.iter().sum();

    let mut hue;
    let mut iteration;

    let mut pixels = HashMap::new();
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            iteration = iteration_counts[&(w, h)];
            hue = 0.0;
            if iteration < MAX_ITER {
                for i in 0..iteration {
                    hue += iter_per_pixel[i as usize] as f64 / total as f64;
                }
            }

            hue *= 2.0;
            if hue > 1.0 {
                hue -= 1.0
            }

            let colour = Hsl::new(
                360.0 * hue,
                100.0,
                if iteration < MAX_ITER { 40.0 } else { 0.0 },
                None,
            );
            //let colour = prisma::Rgb::from_color(&colour);
            let colour = Rgb::from(colour);
            //let colour = colour.encode(SrgbEncoding);
            let red: f64 = colour.red();
            let blue: f64 = colour.blue();
            let green: f64 = colour.green();

            let red = red.round() as u8;
            let blue = blue.round() as u8;
            let green = green.round() as u8;
            canvas.set_draw_color(Color::RGB(red, green, blue));
            canvas
                .draw_point(Point::new(w, h))
                .expect("failed to draw pixel");
            pixels.insert((w, h), [red, green, blue]);
            //println!("{}", hue[&(w, h)])
        }
        canvas.present();
    }

    for w in 0..WIDTH as u32 {
        for h in 0..HEIGHT as u32 {
            image.put_pixel(w, h, image_rgb(pixels[&(w as i32, h as i32)]))
        }
    }
    image.save("output.png").unwrap();
    println!("image saved");
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
