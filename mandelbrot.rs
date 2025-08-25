// rustc mandelbrot.rs -o bin/mandelbrot; ./bin/mandelbrot > output.ppm
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Complex {
    r: f64,
    i: f64
}

impl ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
         return Complex{
            r: self.r*rhs.r - self.i*rhs.i, 
            i: self.r*rhs.i + self.i*rhs.r
        };
    }
}

impl ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        return Complex{
            r: self.r + rhs.r,
            i: self.i + rhs.i
        };
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real=self.r, imag=self.i)
    }
}

fn main() {
    const IMAGE_WIDTH: u32 = 768;
    const IMAGE_HEIGHT: u32 = 768;
    const MAX_BRIGHTNESS: u32 = 255;
    const ITERATIONS: u32 = 100;

    print!("P3\n{} {}\n{}\n", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_BRIGHTNESS);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
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
                print!("{} {} {}\n", 0, 0, 0);
            }
            else {
                let col: u32 = ((iter as f64) / (ITERATIONS as f64) * (MAX_BRIGHTNESS as f64)) as u32;
                print!("{} {} {}\n", col, 0, 0);
            }
        }
    }
}
