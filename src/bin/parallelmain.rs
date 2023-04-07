use colorsys::{Hsl, Rgb};
use image::{Rgb as image_rgb, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::Point;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time;

fn mandelbrot(coord: Complex<f64>, max_iter: u64) -> u64 {
    let mut iteration = 0;
    let mut z = Complex::new(0.0, 0.0);
    while z.abs() <= 4.0 && iteration < max_iter {
        z = z * z + coord;
        iteration += 1
    }
    iteration
}

const ZOOM: f64 = 1.0 / 4.2e-12; // / 0.000007456880595481421; // 1.0 for full set;
const MAX_ITER: u64 = 10000;
const X_OFFSET: f64 = 0.3369844464873; //0.0 for full set;
const Y_OFFSET: f64 = 0.0487782196791; //0.0 for full set;
const MIN_X: f64 = -2.0; //-2.0 recommended
const MAX_X: f64 = 1.0; //1.0 recommended
const MIN_Y: f64 = -1.2; //-1.2 recommended
const MAX_Y: f64 = 1.2; // 1.2 recommended

const SIZE: f64 = 1000.0; //500 is recommended

const WIDTH: i32 = (SIZE * (MAX_X - MIN_X)) as i32;
const HEIGHT: i32 = (SIZE * (MAX_Y - MIN_Y)) as i32;

fn main() {
    let mut iteration_counts = HashMap::new();
    let mut iter_per_pixel: [u32; MAX_ITER as usize + 1] = [0; MAX_ITER as usize + 1];
    let mut iteration: u64;

    let time_taken = time::Instant::now();
    print!("Started");

    let mut coords: Vec<(i32, i32)> = Vec::new();

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            //let x0 = (((((MAX_X - MIN_X) / WIDTH as f64) * w as f64) + MIN_X) / ZOOM) + X_OFFSET;
            //let y0 = (((((MAX_Y - MIN_Y) / HEIGHT as f64) * h as f64) + MIN_Y) / ZOOM) + Y_OFFSET;
            coords.push((w, h))
        }
    }

    let iterationmap: Vec<_> = coords
        .par_iter()
        .map(|&i| {
            if i.0 == i.1 {
                print!(
                    "\rTime so far: {:?} idk there should be like {} of these messages",
                    time_taken.elapsed(),
                    HEIGHT
                )
            };
            (
                mandelbrot(
                    Complex::new(
                        ((((MAX_X - MIN_X) / WIDTH as f64) * i.0 as f64) + MIN_X) / ZOOM + X_OFFSET,
                        ((((MAX_Y - MIN_Y) / HEIGHT as f64) * i.1 as f64) + MIN_Y) / ZOOM
                            + Y_OFFSET,
                    ),
                    MAX_ITER,
                ),
                (i.0, i.1),
            )
        })
        .collect();

    let mut coords: (i32, i32);


    for i in iterationmap {
        iteration = i.0;
        coords = i.1;
        if iteration < MAX_ITER {
            iter_per_pixel[iteration as usize] += 1;
        }
        iteration_counts.insert(coords, iteration);
    }


    let total: u32 = iter_per_pixel.par_iter().sum();

    println!();

    //println!("{:?}",iteration_counts.iter());

    let pixels: HashMap<(i32, i32), [u8; 3]> = iteration_counts.par_iter().map(|i| {
        let iteration = i.1.to_owned();
        let currentcoords = i.0;
        let mut hue = 0.0;

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

        let colour = Rgb::from(colour);
        let red: f64 = colour.red();
        let blue: f64 = colour.blue();
        let green: f64 = colour.green();

        let red = red.round() as u8;
        let blue = blue.round() as u8;
        let green = green.round() as u8;

        (currentcoords.to_owned(), [red, green, blue])

    }).collect();

    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for w in 0..WIDTH as u32 {
        for h in 0..HEIGHT as u32 {
            image.put_pixel(w, h, image_rgb(pixels[&(w as i32, h as i32)]))
        }
    }
    image.save("output.png").unwrap();
    println!("\nImage saved as output.png");
    println!("Time taken: {:?}", time_taken.elapsed())
}