/*
 * Copyright 2022 Instituto Superior de Engenharia do Porto
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
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
