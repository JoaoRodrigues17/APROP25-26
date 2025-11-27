/*
 * Copyright 2022 Instituto Superior de Engenharia do Porto
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use num_cpus;

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

    // Create a thread pool with as many threads as logical CPUs
    let n_threads = num_cpus::get();
    let pool = ThreadPool::new(n_threads);

    // Shared accumulator for points outside the set
    let num_outside_global = Arc::new(Mutex::new(0u64));

    // One task per row i
    for i in 0..NPOINTS {
        let num_outside_cloned = Arc::clone(&num_outside_global);

        pool.execute(move || {
            let mut local_outside = 0u64;

            for j in 0..NPOINTS {
                let c = Complex {
                    r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                    i:  1.125 * (j as f64) / (NPOINTS as f64) + EPS,
                };

                local_outside += test_point(c) as u64;
            }

            // Commit this row's result once
            let mut total = num_outside_cloned.lock().unwrap();
            *total += local_outside;
        });
    }

    // Wait for all tasks to finish
    pool.join();

    let num_outside = *num_outside_global.lock().unwrap() as f64;

    let np = NPOINTS as f64;
    let size = np * np;
    let area = 2.0 * 2.5 * 1.125 * ((size - num_outside) / size);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}\n",
        area, error
    );
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
