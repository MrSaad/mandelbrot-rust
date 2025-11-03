use std::ops;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub r: f64,
    pub i: f64
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

impl ops::Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
         return Complex{
            r: self.r*rhs,
            i: self.i*rhs
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