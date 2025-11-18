struct Complex{
    real: f64,
    imaginary: f64,
}

struct Result{
    area: f64,
    error: f64,
}

const NPOINTS: i32 = 1000;

fn main(){
    let mut numoutside = 0;
    let mut c = Complex {
        real: 0.0,
        imaginary: 0.0,
    };

    let (area, error, eps) = (1.0e-5,1.0e-5,1.0e-5);

    println!("{eps}");

}