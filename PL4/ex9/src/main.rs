use nalgebra::DMatrix;
use std::time;

const SIZE: usize = 1000; 


fn multiply_matrix(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = a.len();
    let mut result = vec![vec![0i32; size]; size];

    for i in 0..size {
        for j in 0..size {
            let mut sum = 0;
            for k in 0..size {
                sum += a[i][k] * b[k][j];
            }
            result[i][j] = sum;
        }
    }

    result
}

fn main() {
    println!("Generating {}x{} matrices...", SIZE, SIZE);

    // Heap-allocated arrays
    let mut matrix_a = vec![vec![0i32; SIZE]; SIZE];
    let mut matrix_b = vec![vec![0i32; SIZE]; SIZE];

    // Fill matrices with sample values
    for i in 0..SIZE {
        for j in 0..SIZE {
            matrix_a[i][j] = (i + j) as i32;
            matrix_b[i][j] = ((i * j) % SIZE) as i32;
        }
    }

    // Convert to DMatrix
    let dmatrix_a = DMatrix::from_row_slice(
        SIZE,
        SIZE,
        &(0..SIZE)
            .flat_map(|i| (0..SIZE).map(move |j| (i + j) as i32))
            .collect::<Vec<_>>(),
    );
    let dmatrix_b = DMatrix::from_row_slice(
        SIZE,
        SIZE,
        &(0..SIZE)
            .flat_map(|i| (0..SIZE).map(move |j| ((i * j) % SIZE) as i32))
            .collect::<Vec<_>>(),
    );

    // Measure array multiplication time
    let start = time::Instant::now();
    let _ = multiply_matrix(&matrix_a, &matrix_b);
    let duration = start.elapsed();
    println!("2D vec {}x{} multiplication time: {:?}", SIZE, SIZE, duration);

    // Measure DMatrix multiplication time
    let start = time::Instant::now();
    let _ = &dmatrix_a * &dmatrix_b;
    let duration = start.elapsed();
    println!("DMatrix {}x{} multiplication time: {:?}", SIZE, SIZE, duration);
}
