use colorsys::{Hsl, Rgb};
use image::{Rgb as image_rgb, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;
use std::collections::HashMap;
use std::time;

fn julia(offset: Complex<f64>, coord: Complex<f64>, max_iter: u64) -> u64 {
    let mut iteration = 0;
    let mut z = coord;
    while z.abs() <= 4.0 && iteration < max_iter {
        z = z * z + offset;
        iteration += 1
    }
    iteration
}

const ZOOM: f64 = 1.0 / 0.0002823785222314; // 1.0 for full set;
const MAX_ITER: u64 = 5000;
const X_OFFSET: f64 = 0.053230709144847205; //0.0 for full set;
const Y_OFFSET: f64 = -0.4985179064621003; //0.0 for full set;
const MIN_X: f64 = -2.0; //-2.0 recommended
const MAX_X: f64 = 1.0; //1.0 recommended
const MIN_Y: f64 = -1.2; //-1.2 recommended
const MAX_Y: f64 = 1.2; // 1.2 recommended

const JULIA_X_OFFSET: f64 = 0.29565408252853365;
const JULIA_Y_OFFSET: f64 = -0.4495171202809482;

const SIZE: f64 = 500.0; //500 is recommended

const WIDTH: i32 = (SIZE * (MAX_X - MIN_X)) as i32;
const HEIGHT: i32 = (SIZE * (MAX_Y - MIN_Y)) as i32;

fn main() {
    let mut iteration_counts = HashMap::new();
    let mut iter_per_pixel: [u32; MAX_ITER as usize + 1] = [0; MAX_ITER as usize + 1];
    let mut iteration: u64;

    let time_taken = time::Instant::now();
    print!("Started");
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let x0 = (((((MAX_X - MIN_X) / WIDTH as f64) * w as f64) + MIN_X) / ZOOM) + X_OFFSET;
            let y0 = (((((MAX_Y - MIN_Y) / HEIGHT as f64) * h as f64) + MIN_Y) / ZOOM) + Y_OFFSET;
            let scaledcoords = Complex::new(x0, y0);
            iteration = julia(
                Complex::new(JULIA_X_OFFSET, JULIA_Y_OFFSET),
                scaledcoords,
                MAX_ITER,
            );

            if iteration < MAX_ITER {
                iter_per_pixel[iteration as usize] += 1;
            }
            iteration_counts.insert((w, h), iteration);
        }
        print!("\rLoop 1 {w}/{WIDTH}", w = w + 1);
    }

    let total: u32 = iter_per_pixel.iter().sum();
    let mut pixels = HashMap::new();
    let mut hue: f64;

    println!();

    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            iteration = iteration_counts[&(w, h)];
            hue = 0.0;

            if iteration < MAX_ITER {
                for i in 0..iteration {
                    hue += iter_per_pixel[i as usize] as f64 / total as f64;
                }
            }

            // makes colours look better
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

            pixels.insert((w, h), [red, green, blue]);
        }
        print!("\rLoop 2 {w}/{WIDTH}", w = w + 1);
    }

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
