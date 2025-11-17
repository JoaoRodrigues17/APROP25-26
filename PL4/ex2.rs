// #[derive(Debug, Clone, Copy)]

pub struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl ComplexNumber{
    pub fn init(real: f64, imaginary: f64) -> ComplexNumber {
        ComplexNumber{
            real,
            imaginary,
        } 
    }

    pub fn add(&self, element: &ComplexNumber) -> ComplexNumber {
        ComplexNumber{
            real: self.real + element.real,
            imaginary: self.imaginary + element.imaginary,
        }  
    }

    pub fn sub(&self, element: &ComplexNumber) -> ComplexNumber {
        ComplexNumber{
            real: self.real - element.real,
            imaginary: self.imaginary - element.imaginary,
        }
    }

    pub fn multiply(&self, element: &ComplexNumber) -> ComplexNumber {
        let real_part = self.real*element.real - self.imaginary*element.imaginary;
        let imaginary_part = self.real*element.imaginary + self.imaginary*element.real;
        ComplexNumber{
            real: real_part,
            imaginary: imaginary_part,
        }
    }

    pub fn divide(&self, element: &ComplexNumber) -> ComplexNumber {
         let denom = element.real * element.real + element.imaginary * element.imaginary;
        let real_part = (self.real * element.real + self.imaginary * element.imaginary) / denom;
        let imaginary_part = (self.imaginary * element.real - self.real * element.imaginary) / denom;
        ComplexNumber{
            real: real_part,
            imaginary: imaginary_part,
        }
    }

    pub fn print(&self) {
        if self.imaginary >= 0.0 {
            println!("{} + {}i", self.real, self.imaginary);
        } else {
            println!("{} - {}i", self.real, -self.imaginary);
        }
    }
}

//main.rs
// use rand::Rng; 

mod complex_number;
use complex_number::ComplexNumber;

fn main() {
    // let a = ComplexNumber::init(4.0, 5.0);
    // let b = ComplexNumber::init(2.0, -3.0);

    // Generate random f64 between -10.0 and 10.0 for both parts
    let mut rng = rand::thread_rng();
    let a = ComplexNumber::init(rng.gen_range(-10.0..=10.0), rng.gen_range(-10.0..=10.0));
    let b = ComplexNumber::init(rng.gen_range(-10.0..=10.0), rng.gen_range(-10.0..=10.0));


    let sum = a.add(&b);
    print!("Sum: ");
    sum.print();

    let diff = a.sub(&b);
    print!("Difference: ");
    diff.print();

    let product = a.multiply(&b);
    print!("Product: ");
    product.print();

    let quotient = a.divide(&b);
    print!("Quotient: ");
    quotient.print();
}
