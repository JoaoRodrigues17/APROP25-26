//In these exercises, we'll be practicing some core concepts of rust.
//The focus is in learning the language, not having the best performing code for a real world scenario
//Therefore, some best practices (checking if array is empty, repeating declarations, p.e.) are not followed for simplicity purposes
use std::thread;
use std::sync::{Arc, Mutex};

const NUM_THREADS:usize = 4;


// functions for a)
fn max(vec: &Vec<i32>) -> i32{

    let chunk_size = (vec.len()+NUM_THREADS-1) / NUM_THREADS;
    let mut handles = Vec::new();

    for chunk in vec.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let handle = thread::spawn(move || {
            let mut max_val = chunk[0];
            for &x in &chunk[1..] {
                if x > max_val {
                    max_val = x;
                }
            }
            max_val
        });
        handles.push(handle);
    }

    let mut global_max = i32::MIN;
    for handle in handles {
        let local_max = handle.join().unwrap();
        if local_max > global_max {
            global_max = local_max;
        }
    }

    global_max

}

fn min(vec: &Vec<i32>) -> i32{
    let chunk_size = (vec.len()+NUM_THREADS-1) / NUM_THREADS;
    let mut handles = Vec::new();

    for chunk in vec.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let handle = thread::spawn(move || {
            let mut min_val = chunk[0];
            for &x in &chunk[1..] {
                if x < min_val {
                    min_val = x;
                }
            }
            min_val
        });
        handles.push(handle);
    }

    let mut global_min = i32::MAX;
    for handle in handles {
        let local_min = handle.join().unwrap();
        if local_min < global_min {
            global_min = local_min;
        }
    }

    global_min
    
}

fn avg(vec: &Vec<i32>) -> f32{
    let chunk_size = (vec.len()+NUM_THREADS-1) / NUM_THREADS;
    let mut handles = Vec::new();

    for chunk in vec.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let handle = thread::spawn(move || {
            chunk.iter().sum::<i32>() as f32
        });
        handles.push(handle);
    }
    let mut total_sum = 0.0;
    for handle in handles {
        let local_sum = handle.join().unwrap();
        total_sum += local_sum;
    }
    total_sum / (vec.len() as f32)

}

//functions for b)
fn median(vec: &Vec<i32>) -> Option<i32>{

    for i in 1..vec.len() {
        if vec[i] < vec[i-1] {
            return None;
        }
    }

    let mid = vec.len() / 2;
    if vec.len() % 2 == 0{
        Some((vec[mid-1] + vec[mid]) / 2)
    } else {
        Some(vec[mid])
    }
}

//functions for c)
fn vec_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Option<Vec<i32>>{
    if v1.len() != v2.len(){
        return None;
    }
    
    let len = v1.len();
    let v3 = Arc::new(Mutex::new(vec![0; len]));

    let chunk_size = (len+NUM_THREADS-1) / NUM_THREADS;

    let mut handles = Vec::new();

    for thread_id in 0..NUM_THREADS {
        let v1 = v1.clone();
        let v2 = v2.clone();
        let v3 = Arc::clone(&v3);

        handles.push(thread::spawn(move || {
            let mut v3 = v3.lock().unwrap();
            for i in thread_id * chunk_size..(thread_id + 1) * chunk_size {
                v3[i] = v1[i] + v2[i];
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    Some(Arc::try_unwrap(v3).ok()?.into_inner().ok()?)
}

//functions for d)
fn quicksort(v1: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let m = split(v1, low, high);
        if m > 0 {
            quicksort(v1, low, m - 1);
        }
        quicksort(v1, m + 1, high);
    }
}

fn split(v1: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = v1[high];
    let mut m = low;

    for i in low..high {
        if v1[i] < pivot {
            v1.swap(i, m);
            m += 1;
        }
    }

    v1.swap(m, high);
    m
}


//functions for e)
fn bubblesort(v1: &mut Vec<i32>){
    let mut changes = 1;
    while changes != 0 {
        changes = 0;
        for i in 0..v1.len()-1{
            if v1[i] > v1[i+1] {
                let tmp = v1[i];
                v1[i] = v1[i+1];
                v1[i+1] = tmp;
                changes+=1;
            }
        }
    }
}

//functions for f)
fn median_with_sorting(vec: &mut Vec<i32>) -> i32 {
    let mut sorted = true;
    for i in 1..vec.len() {
        if vec[i] < vec[i-1] {
            sorted = false;
        }
    }

    if !sorted {
        bubblesort(vec);
    }

    let mid = vec.len() / 2;
    if vec.len() % 2 == 0{
        (vec[mid-1] + vec[mid]) / 2
    } else {
        vec[mid]
    }

}

fn main(){
    // a) max, min and avg functions
    println!("A) max, min and avg functions");
    let v1 = vec![10,5,82,12,56,39,73,8];
    println!("v1: {:?}",v1);

    let max = max(&v1);
    let min = min(&v1);
    let avg = avg(&v1);
    println!("v1 max: {max}");
    println!("v1 min: {min}");
    println!("v1 avg: {avg}\n");

    // b) median (fail if not ordered)  --using if let and match clauses for practise
    println!("B) median (fail if not ordered)");
    let v1 = vec![10,5,82,12,56,39,73,8];
    let v2 = vec![1,5,6,8,9,14,20,25];
    println!("v1: {:?}",v1);
    println!("v2: {:?}\n",v2);

    if let Some(median) = median(&v1){
        println!("v1 median: {median}");
    }else{
        println!("Couldn't do median, v1 is not ordered!")
    }

    match median(&v2){
        Some(median) => println!("v2 median: {median}\n"),
        None => println!("Couldn't do median, v2 is not ordered!\n"),
    }

    // c) Vector sum of 2 Vectors (fail if different sizes). Using match because it looks way more readable
    println!("C) Vector sum of 2 Vectors (fail if different sizes)");
    let v1 = vec![10,5,82,12,56,39,73,8];
    let v2 = vec![1,5,6,8,9,14,20,25];
    let v3 = vec![1,5,9,4];
    println!("v1: {:?}",v1);
    println!("v2: {:?}",v2);
    println!("v3: {:?}\n",v3);

    match vec_sum(&v1,&v2){
        Some(v3) => println!("vec_sum v1 + v2: {:?}",v3),
        None => println!("Couldn't do sum v1 + v2, Different size"),
    }

    match vec_sum(&v2,&v3){
        Some(v3) => println!("vec_sum v2 + v3: {:?}\n",v3),
        None => println!("Couldn't do sum v2 + v3, Different size\n"),
    }

    // d) Sorting a Vector with Quicksort
    println!("D) Sorting a Vector with Quicksort");
    let mut v1 = vec![10,5,82,12,56,39,73,8];
    println!("v1: {:?}",v1);
    let len = v1.len();
    quicksort(&mut v1,0,len-1);
    println!("Using quicksort...");
    println!("Sorted v1: {:?}\n",v1);

    //e) Sorting a Vector with BubbleSort
    println!("E) Sorting a Vector with BubbleSort");
    let mut v1 = vec![10,5,82,12,56,39,73,8];
    println!("v1: {:?}",v1);
    bubblesort(&mut v1);
    println!("Using bubblesort...");
    println!("Sorted v1: {:?}\n",v1);

    //f) Median with Sorting
    println!("F) Median with sorting");
    let mut v1 = vec![10,5,82,12,56,39,73,8];
    println!("v1: {:?}",v1);
    println!("v1 median (after sort): {:?}\n",median_with_sorting(&mut v1));

}