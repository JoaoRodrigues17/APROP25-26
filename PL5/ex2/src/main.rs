#[derive(Copy, Clone)]
struct Complex {
    r: f64,
    i: f64,
}
const NPOINTS: u32 = 1000;
const MAXITER: u32 = 1000;
const EPS: f64 = 1e-5;
fn main() {
    println!("Mandelbrot!");
     let num_outside: i32 = (0..NPOINTS)
        .flat_map(|i| {
            (0..NPOINTS).map(move |j| {
                let c = Complex {
                    r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                    i: 1.125 * (j as f64) / (NPOINTS as f64) + EPS,
                };
                test_point(c)
            })
        })
        .sum();

    let size = NPOINTS * NPOINTS;
    let area = 2.0 * 2.5 * 1.125 * ((size as f64 - num_outside as f64) / size as f64);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}\n",
        area, error
    );
    println!("num_outside = {}", num_outside);
}

fn test_point(c: Complex) -> i32 {
    let mut z = c.clone();
    for _ in 0..MAXITER {
        let temp = (z.r * z.r) - (z.i * z.i) + c.r;
        z.i = z.r * z.i * 2.0 + c.i;
        z.r = temp;
        if z.r * z.r + z.i * z.i > 4.0 {
            return 1;
        }
    }
    0
}
