use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};

const NUM_THREADS:usize = 4;

fn mode(v1: &Vec<i32>) -> i32 {
    // Parallel frequency count
    let map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();
    let chunk_size = (v1.len()+NUM_THREADS-1) / NUM_THREADS;
    for chunk in v1.chunks(chunk_size){
        let chunk = chunk.to_owned();
        let map = Arc::clone(&map);
        let handle = thread::spawn( move || {
            for &elem in &chunk {
                *map.lock().unwrap().entry(elem).or_insert(0) += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    let map = Arc::try_unwrap(map).unwrap().into_inner().unwrap();

    // Parallel Find max frequency
    let chunk_size = (map.len()+NUM_THREADS-1) / NUM_THREADS;
    let mut handles = Vec::new();
    let entries: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    for chunk in entries.chunks(chunk_size){
        let chunk = chunk.to_owned();
        let handle = thread::spawn( move || {
            let mut local_max = (0,0); // (value, count)
            for (key, value) in chunk {
                if value > local_max.1 {
                    local_max = (key, value);
                }
            }
            local_max
        });
        handles.push(handle);
    }
    let mut res = (i32::MIN, i32::MIN);
    for handle in handles {
        let local_max = handle.join().unwrap();
        if local_max.1 > res.1 {
            res = local_max;
        }
    }

    res.0
}



fn main(){
    let v1 = vec![10,52,73,82,12,56,52,73,12,52,8,73,39,73,8,15];
    println!("v1: {:?}",v1);
    println!("v1 mode: {:?}",mode(&v1));
}