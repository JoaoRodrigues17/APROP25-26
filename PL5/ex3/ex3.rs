//The function declaration is higly dependant on the definition of the matrixes
//For this to be dynamic, we would use Vec


fn multiply_matrix(a:[[i32; 3]; 3] ,b:[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            let mut sum = 0;
            for k in 0..3{
                sum += a[i][k] * b[k][j];
            }
            result[i][j] = sum;
        }
    }
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