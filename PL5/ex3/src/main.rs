use complex::Complex;
use rayon;
use rayon::prelude::*;
mod complex;


//a) max, min, average of vectors with Complex numbers
fn max_complex(v: &Vec<Complex>) -> Option<&Complex>{
    v.par_iter().max_by(|a, b| {
        a.module().partial_cmp(&b.module()).unwrap()
    })
}

fn min_complex(v: &Vec<Complex>) -> Option<&Complex>{
    v.par_iter().min_by(|a, b| {
        a.module().partial_cmp(&b.module()).unwrap()
    })
}

fn avg_complex(v: &Vec<Complex>) -> Complex{
    let sum = complex_vec_sum(v);
    let len = v.len() as f64;
    Complex::new(sum.real / len, sum.imaginary / len)
}

//b) sum of the Complex elements in the vector (This could be done with iterators if Sum trait was implemented, but we haven't learned that)
fn complex_vec_sum(v: &Vec<Complex>) -> Complex{
    v.par_iter().fold(||Complex::new(0.0, 0.0), |acc, x| acc.add(&x)).reduce(||Complex::new(0.0, 0.0), |a, b| a.add(&b))
}


//c) vector of modules of the original Complex numbers vector
fn modules(v:&Vec<Complex>) -> Vec<f64>{
    v.par_iter().map(Complex::module).collect()
}

fn main(){
    let v = vec![
    Complex::new(4.0, 4.0),
    Complex::new(1.0, -3.0),
    Complex::new(-2.5, 7.1),
    Complex::new(0.0, -8.0),
    Complex::new(3.14, 2.71),
    ];
    let v1 = v.iter();
    
    println!("\nVEC:");
    for elem in v1 {
        println!("{}, module: {:.2}",elem,elem.module());

    }
    println!("\n****************************************************\n");
    println!("Vec max: {}", max_complex(&v).unwrap());
    println!("Vec min: {}", min_complex(&v).unwrap());
    println!("Vec avg: {}", avg_complex(&v));
    println!("Modules of vec: {:.2?}", modules(&v));
    println!("Sum of vec elements: {:.2}\n", complex_vec_sum(&v));
}