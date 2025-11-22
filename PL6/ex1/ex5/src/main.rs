use nalgebra::DMatrix;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::io;
use std::thread;
use std::sync::{Arc, Mutex};

//This main function:
//Asks for the size of the matrix
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

    // Initialize Dynamic matrices (They don't grow or shrink like Vec<T>)
    let a = Arc::new(Mutex::new(DMatrix::<i32>::zeros(n, n)));
    let b = Arc::new(Mutex::new(DMatrix::<i32>::zeros(n, n)));

    let mut handles = Vec::new();
    // Fill with random values
    // Parallelized by row
    for i in 0..n {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move ||{
            let mut rng = StdRng::from_entropy();
            for j in 0..n {
                let mut matrix_a = a.lock().unwrap(); 
                matrix_a[(i, j)] = rng.gen_range(-10..10);
                let mut matrix_b = b.lock().unwrap(); 
                matrix_b[(i, j)] = rng.gen_range(-10..10);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let a = Arc::try_unwrap(a).unwrap().into_inner().unwrap();
    let b = Arc::try_unwrap(b).unwrap().into_inner().unwrap();

    //println! works because the display trait is implemented in DMatrix
    println!("Matrix A:{}", a);
    println!("Matrix B:{}", b);

    // Matrix multiplication provided by DMatrix
    let c = &a * &b;

    println!("A * B ={}", c);
}