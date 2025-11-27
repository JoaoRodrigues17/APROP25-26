use std::sync::Arc;
use std::thread;
use parking_lot::Mutex;

fn multiply_matrices_without_mutex(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();

    // Check matrix dimensions
    assert!(b.len() == n, "Incompatible matrix dimensions!");

    let mut result = vec![vec![0.0; p]; m];
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();

    assert!(b.len() == n, "Incompatible matrix dimensions!");

    // parking_lot::Mutex instead of std::sync::Mutex
    let result = vec![vec![0.0_f64; p]; m];
    let result = Arc::new(Mutex::new(result));

    let a = Arc::new(a.clone());
    let b = Arc::new(b.clone());

    let mut handles = Vec::new();

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

            // parking_lot::Mutex::lock() returns a guard similarly
            let mut res_guard = result_cloned.lock();
            res_guard[i] = row;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // parking_lot::Mutex<T> exposes into_inner on the guard, but not on the mutex itself,
    // so just clone out of the Arc+Mutex in this simple example.
    let res_locked = result.lock().clone();
    res_locked
}

fn print_matrix(mat: &Vec<Vec<f64>>) {
    for row in mat {
        for val in row {
            print!("{:.2} ", val);
        }
        println!();
    }
}

fn main() {
    let a = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
    ];

    let b = vec![
        vec![7.0, 8.0],
        vec![9.0, 10.0],
        vec![11.0, 12.0],
    ];

    let result = multiply_matrices(&a, &b);

    println!("Result of matrix multiplication:");
    print_matrix(&result);
}
