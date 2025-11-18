//In these exercises, we'll be practicing some core concepts of rust.
//The focus is in learning the language, not having the best performing code for a real world scenario
//Therefore, some best practices (checking if array is empty, repeating declarations, p.e.) are not followed for simplicity purposes


// functions for a)
fn max(vec: &Vec<i32>) -> i32{
    let mut max = vec[0];
    for &elem in vec.iter().skip(1) {
        if elem > max {
            max = elem;
        }
    }
    max

}

fn min(vec: &Vec<i32>) -> i32{
    let mut min = vec[0];
    for &elem in vec.iter().skip(1) {
        if elem < min {
            min = elem;
        }
    }
    min
}

fn avg(vec: &Vec<i32>) -> i32{
    let mut sum = 0;
    for &elem in vec {
        sum += elem;
    }
    sum / (vec.len() as i32)
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

    let mut v3 = Vec::new();
    for i in 0..v1.len(){
        v3.push(v1[i] + v2[i]);
    }

    Some(v3)

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