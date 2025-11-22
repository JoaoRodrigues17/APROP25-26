use nalgebra::DMatrix;
use rand::Rng;
use std::io;

//This main function:
//Asks for the size of the matrix
//
fn main() {
    println!("Enter the size n for an n x n matrix:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: usize = input
        .trim()
        .parse()
        .expect("Please enter a valid positive integer");

    let mut rng = rand::thread_rng();

    // Initialize Dynamic matrices (They don't grow or shrink like Vec<T>)
    let mut a = DMatrix::<i32>::zeros(n, n);
    let mut b = DMatrix::<i32>::zeros(n, n);

    // Fill with random values
    for i in 0..n {
        for j in 0..n {
            a[(i, j)] = rng.gen_range(-10..10);
            b[(i, j)] = rng.gen_range(-10..10);
        }
    }

    //println! works because the display trait is implemented in DMatrix
    println!("Matrix A:{}", a);
    println!("Matrix B:{}", b);

    // Matrix multiplication provided by DMatrix
    let c = &a * &b;

    println!("A * B ={}", c);
}