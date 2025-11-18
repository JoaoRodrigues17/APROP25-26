use std::collections::HashMap;

fn mode(v1: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &elem in v1 {
        *map.entry(elem).or_insert(1) += 1;
    }
    let mut mode = v1[0];
    let mut max_count = 0;

    for (&elem, &count) in &map {
        if count > max_count {
            max_count = count;
            mode = elem;
        }
    }
    mode
}



fn main(){
    let v1 = vec![10,52,82,12,56,52,73,12,52,8,73,39,73,8];
    println!("v1: {:?}",v1);
    println!("v1 mode: {:?}",mode(&v1));
}