use std::thread;
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use rayon;
use rayon::prelude::*;
use std::time::Instant;


#[derive(Copy, Clone)]
struct Complex {
    r: f64,
    i: f64,
}
const EPS: f64 = 1e-5;

//Settings for test
const NPOINTS: u32 = 100;
const MAXITER: u32 = 1000;
const NRUNS: u32 = 5;

fn mandelbrot_seq(){
    println!("Sequential Mandelbrot!");
    let mut num_outside = 0;
    for i in 0..NPOINTS {
        for j in 0..NPOINTS {
            let c = Complex {
                r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                i: 1.125 * (j as f64) / (NPOINTS as f64) + EPS,
            };

            num_outside += test_point(c);
        }
    }

    let np = NPOINTS as f64;
    let size = np * np;
    let area = 2.0 * 2.5 * 1.125 * ((size - num_outside as f64) / size);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}",
        area, error
    );
    println!("num_outside = {}\n", num_outside);
}

fn mandelbrot_threadpool(){
    println!("Mandelbrot with ThreadPool!");
    let n_workers = thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(n_workers);
    let num_outside =  Arc::new(Mutex::new(0));
    for i in 0..NPOINTS {
        let num_outside = num_outside.clone();
        pool.execute(move || {
            let mut local = 0;
            for j in 0..NPOINTS {
                let c = Complex {
                    r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                    i: 1.125 * (j as f64) / (NPOINTS as f64) + EPS,
                };
                local += test_point(c);                
            }
            let mut num_outside = num_outside.lock().unwrap();  
                *num_outside += local;

        });
    }
    pool.join();

    let num_outside = *num_outside.lock().unwrap();

    let size = NPOINTS * NPOINTS;
    let area = 2.0 * 2.5 * 1.125 * ((size as f64 - num_outside as f64) / size as f64);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}\n",
        area, error
    );
    println!("num_outside = {}", num_outside);
}

fn mandelbrot_rayon(){
    println!("Mandelbrot with Rayon!");
    let num_outside: i32 = (0..NPOINTS).into_par_iter().map(|i| {
        let mut local_count = 0;
        for j in 0..NPOINTS {
            let c = Complex {
                r: -2.0 + 2.5 * (i as f64) / (NPOINTS as f64) + EPS,
                i: 1.125 * (j as f64) / (NPOINTS as f64) + EPS,
            };
            local_count += test_point(c);
        }
        local_count
    }).sum();

    let np = NPOINTS as f64;
    let size = np * np;
    let area = 2.0 * 2.5 * 1.125 * ((size - num_outside as f64) / size);
    let error = area / NPOINTS as f64;

    println!(
        "Area of Mandlebrot set = {:12.8} +/- {:12.8}",
        area, error
    );

    println!("num_outside = {}\n", num_outside);
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

fn main() {

    let mut seq_time = std::time::Duration::new(0, 0);
    let mut tp_time = std::time::Duration::new(0, 0);
    let mut rayon_time = std::time::Duration::new(0, 0);

    for _ in 0..NRUNS {
        let start = Instant::now();
        mandelbrot_seq();
        seq_time += start.elapsed();

        let start = Instant::now();
        mandelbrot_threadpool();
        tp_time += start.elapsed();

        let start = Instant::now();
        mandelbrot_rayon();
        rayon_time += start.elapsed();
    }
    seq_time /= NRUNS;
    tp_time /= NRUNS;
    rayon_time /= NRUNS;

    println!("**************************\nResults (Average of {} runs):", NRUNS);
    println!("Sequential execution: {}", seq_time.as_secs_f64());
    println!("ThreadPool execution: {}", tp_time.as_secs_f64());
    println!("Rayon execution: {}", rayon_time.as_secs_f64());
}