// fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
//     let m = a.len();
//     let n = a[0].len();
//     let p = b[0].len();

//     // Check matrix dimensions
//     assert!(b.len() == n, "Incompatible matrix dimensions!");

//     let mut result = vec![vec![0.0; p]; m];
//     for i in 0..m {
//         for j in 0..p {
//             for k in 0..n {
//                 result[i][j] += a[i][k] * b[k][j];
//             }
//         }
//     }
//     result
// }

// use nalgebra::DMatrix;

// fn multiply_dmatrices(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
//     // Standard DMatrix multiplication; panics if incompatible sizes
//     a * b
// }

// fn print_matrix(mat: &Vec<Vec<f64>>) {
//     for row in mat {
//         for val in row {
//             print!("{:.2} ", val);
//         }
//         println!();
//     }
// }

// fn main() {
//     let a = vec![
//         vec![1.0, 2.0, 3.0],
//         vec![4.0, 5.0, 6.0],
//     ];

//     let b = vec![
//         vec![7.0, 8.0],
//         vec![9.0, 10.0],
//         vec![11.0, 12.0],
//     ];

//     let result = multiply_matrices(&a, &b);

//     println!("Result of matrix multiplication:");
//     print_matrix(&result);

//     let c = DMatrix::<f64>::from_row_slice(2, 3, &[
//         1.0, 2.0, 3.0,
//         4.0, 5.0, 6.0,
//     ]);

//     let d = DMatrix::<f64>::from_row_slice(3, 2, &[
//         7.0, 8.0,
//         9.0, 10.0,
//         11.0, 12.0,
//     ]);

//     let result_dynamic = multiply_dmatrices(&c, &d);

//     println!("Result of matrix multiplication:\n{}", result_dynamic);

// }

use std::sync::Arc;
use std::thread;
use parking_lot::Mutex;

fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();

    assert_eq!(b.len(), n, "Incompatible matrix dimensions!");

    let a = Arc::new(a.clone());
    let b = Arc::new(b.clone());

    // Shared result protected by parking_lot::Mutex
    let result = vec![vec![0.0_f64; p]; m];
    let result = Arc::new(Mutex::new(result));

    let mut handles = Vec::with_capacity(m);
    for i in 0..m {
        let a_cloned = Arc::clone(&a);
        let b_cloned = Arc::clone(&b);
        let result_cloned = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let n = a_cloned[0].len();
            let p = b_cloned[0].len();
            let mut row = vec![0.0_f64; p];

            for j in 0..p {
                let mut acc = 0.0;
                for k in 0..n {
                    acc += a_cloned[i][k] * b_cloned[k][j];
                }
                row[j] = acc;
            }

            // Minimal critical section: write full row once
            let mut res = result_cloned.lock();
            res[i] = row;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // Clone out of Arc<Mutex<_>> for simplicity
    result.lock().clone()
}
