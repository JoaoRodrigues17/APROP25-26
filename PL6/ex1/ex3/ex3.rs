//The function declaration is higly dependant on the definition of the matrixes
//For this to be dynamic, we would use Vec

use std::thread;
use std::sync::{Arc, Mutex};
// Line wise parallelism
fn multiply_matrix(a:[[i32; 3]; 3] ,b:[[i32; 3]; 3]) -> [[i32; 3]; 3] {

    let result = Arc::new(Mutex::new([[0; 3]; 3]));
    let mut handles = Vec::new();

    for i in 0..3 {
            let result = Arc::clone(&result);
            let handle = thread::spawn(move || {
                for j in 0..3 {
                    let mut sum = 0;
                    for k in 0..3{
                        sum += a[i][k] * b[k][j];
                    }
                    let mut res = result.lock().unwrap();
                    res[i][j] = sum;
                }
            });
            handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();
    result
}


fn main() {

    // A 3x3 matrix of integers
    let matrix_a: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("MATRIX A");
    for row in matrix_a {
        println!("{:?}", row);
    }
    println!("");
    let matrix_b: [[i32; 3]; 3] = [
        [9, 8, 7],
        [6, 5, 4],
        [3, 2, 1],
    ];
    println!("MATRIX B");
    for row in matrix_b {
        println!("{:?}", row);
    }
    println!("");  
    let result = multiply_matrix(matrix_a, matrix_b);
    println!("MATRIX RESULT");
    for row in result {
        println!("{:?}", row);
    }

}