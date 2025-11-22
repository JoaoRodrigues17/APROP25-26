use complex::Complex;
mod complex;

fn main(){
    let c1 = Complex::new(3.0,4.0);
    let c2 = Complex::new(1.0,2.0);
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