use std::collections::HashMap;

fn mode(v1: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &elem in v1 {
        *map.entry(elem).or_insert(1) += 1;
    }
    
    match map.iter().max_by_key(|(_, count)| *count) {
        Some((value, _)) => *value,
        None => 0
    }
}



fn main(){
    let v1 = vec![10,52,73,82,12,56,52,73,12,52,8,73,39,73,8];
    println!("v1: {:?}",v1);
    println!("v1 mode: {:?}",mode(&v1));
}