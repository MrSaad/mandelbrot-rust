// cargo run --release > output.ppm
mod complex;
use complex::Complex;
use rayon::prelude::*;

const IMAGE_WIDTH: u32 = 5000;
const IMAGE_HEIGHT: u32 = 5000;
const MAX_BRIGHTNESS: u32 = 255;
const ITERATIONS: u32 = 200;


fn mandelbrot_pixel(i: u32, j: u32) -> String {
    let x = (i as f64) / (IMAGE_WIDTH as f64) * 3.0 - 2.0;
    let y = (j as f64) / (IMAGE_HEIGHT as f64) * 3.0 - 1.5;

    let c = Complex{r: x as f64, i: y as f64};
    let mut z = Complex{r: 0.0, i: 0.0};

    let mut iter = 1;
    while (z.r*z.r + z.i*z.i <= 4.0) && (iter < ITERATIONS) {
        z = z * z + c;
        iter += 1;
    }

    if iter == ITERATIONS {
        format!("{} {} {}", 0, 0, 0)
    }
    else {
        let col: u32 = ((iter as f64) / (ITERATIONS as f64) * (MAX_BRIGHTNESS as f64)) as u32;
        format!("{} {} {}", col, 0, 0)
    }
}

fn main() {
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_BRIGHTNESS);

    let pixels: Vec<(u32, u32)> = (0..IMAGE_HEIGHT)
        .flat_map(|j| (0..IMAGE_WIDTH).map(move |i| (i, j)))
        .collect();
    
    let colors: Vec<String> = pixels
        .par_iter()
        .map(|&(i, j)| mandelbrot_pixel(i, j))
        .collect();
    
    for color in colors {
        println!("{}", color);
    }
}
