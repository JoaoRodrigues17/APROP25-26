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

use rayon::prelude::*;

fn main() {
    println!("Mandelbrot!");

    let np = NPOINTS as usize;
    let total_points = np * np;

    // Parallel over all (i, j) points, flattened as a single range
    let num_outside: i32 = (0..total_points)
        .into_par_iter()
        .map(|idx| {
            let i = (idx / np) as u32;
            let j = (idx % np) as u32;

            let c = Complex {
                r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                i: 1.125 * (j as f64) / (NPOINTS as f64) + EPS,
            };

            test_point(c) // 0 or 1
        })
        .sum(); // parallel reduction

    let npf = NPOINTS as f64;
    let size = npf * npf;
    let area = 2.0 * 2.5 * 1.125 * ((size - num_outside as f64) / size);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}\n",
        area, error
    );
}

fn test_point(c: Complex) -> i32 {
    let mut z = c;
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
