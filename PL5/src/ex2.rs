use std::fmt;

struct Complex{
    real: f64,
    imaginary: f64,
}

impl Complex{
    fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    fn subtract(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }

    fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: ((self.real*other.real) - (self.imaginary*other.imaginary)),
            imaginary: (self.real*other.imaginary)+(self.imaginary*other.real),
        }
    }

    fn divide(&self, other: &Complex) -> Option<Complex> {
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

fn main(){
    let c1 = Complex::new(3.0,4.0);
    let c2 = Complex::new(0.0,0.0);
    println!("");
    println!("Complex c1: {}",c1);
    println!("Complex c2: {}\n",c2);
    println!("c1+c2: {}",c1.add(&c2));
    println!("c1-c2: {}",c1.subtract(&c2));
    println!("c1*c2: {}",c1.multiply(&c2));
    match c1.divide(&c2) {
        Some(c3) => println!("c1 / c2 = {}\n", c3),
        None => println!("Division by zero!\n"),
    }
    
}