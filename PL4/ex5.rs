fn multiply_matrices(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

use nalgebra::DMatrix;

fn multiply_dmatrices(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
    // Standard DMatrix multiplication; panics if incompatible sizes
    a * b
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

    let c = DMatrix::<f64>::from_row_slice(2, 3, &[
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
    ]);

    let d = DMatrix::<f64>::from_row_slice(3, 2, &[
        7.0, 8.0,
        9.0, 10.0,
        11.0, 12.0,
    ]);

    let result_dynamic = multiply_dmatrices(&c, &d);

    println!("Result of matrix multiplication:\n{}", result_dynamic);

}
