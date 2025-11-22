use std::fmt;

pub struct Complex{
    real: f64,
    imaginary: f64,
}

impl Complex{
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    pub fn subtract(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }

    pub fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: ((self.real*other.real) - (self.imaginary*other.imaginary)),
            imaginary: (self.real*other.imaginary)+(self.imaginary*other.real),
        }
    }

    pub fn divide(&self, other: &Complex) -> Option<Complex> {
        let denominator = other.real.powi(2) + other.imaginary.powi(2);
        if denominator == 0.0 {
            return None; // Avoid division by zero 
        }

        Some(Complex {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denominator,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denominator,
        })
    }
}


impl fmt::Display for Complex { //Implementing the display trait on "Complex" so that we can use Complex on the println! macro
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imaginary)
        } else {
            write!(f, "{} - {}i", self.real, -self.imaginary)
        }
    }
}