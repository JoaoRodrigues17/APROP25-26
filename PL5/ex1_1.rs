use rand::Rng;
use std::thread;
use std::sync::{mpsc, Arc};

// Ex1 a)
fn max_value(nums: Vec<i32>) -> i32{
    let mut max = nums[0];
    for num in nums {
        if num > max {
            max = num;
        }
    }
    max
}

fn min_value(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    for num in nums {
        if num < min {
            min = num;
        }
    }
    min
}

fn average(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for num in nums {
        sum += num;
        count += 1;
    }
   sum as i32 / count as i32
}

//EX1 b)
fn sort(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    nums
}

fn is_sorted(nums: &Vec<i32>) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

fn median(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    nums = sort(nums);
    if !is_sorted(&nums){
        panic!("ERROR: vector not sorted")
    }
    else{
        if len%2==1{
            nums[len/2] as i32
        }
        else{
            let mid_upper = nums[len / 2];
            let mid_lower = nums[len / 2 - 1];
            (mid_upper as i32 + mid_lower as i32) / 2 as i32
        }
    }

}

//EX1 c)

fn vector_sum(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32>{
    let mut v3: Vec<i32> = vec![];
    for (a, b) in v1.iter().zip(v2.iter()) {
        v3.push(a+b);
    }
    v3
}

// EX1 d)
fn quick_sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr.remove(0);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for item in arr {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left = quick_sort(left);
    let mut sorted_right = quick_sort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
}

// EX1 e)
fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// EX1 f)
fn median(nums: &Vec<i32>) -> i32 {
    let len = nums.len();
    let mut nums_copy = nums.clone();
    if !is_sorted(&nums_copy) {
        nums_copy.sort();
    }
    if len % 2 == 1 {
        nums_copy[len / 2]
    } else {
        let mid_upper = nums_copy[len / 2];
        let mid_lower = nums_copy[len / 2 - 1];
        (mid_upper + mid_lower) / 2
    }
}
    


fn main(){
    // let v: Vec<i32> = (0..100).collect();

    // println!("Vector: {:?}", v);

    let mut rng = rand::thread_rng();
    let v: Vec<i32> = (0..100)
        .map(|_| rng.gen_range(0..=500))
        .collect();
    // println!("{:?}", v);

    let v1: Vec<i32> = (0..100)
        .map(|_| rng.gen_range(0..=500))
        .collect();
    // println!("{:?}", v1);

    let v2: Vec<i32> = (0..100)
        .map(|_| rng.gen_range(0..=500))
        .collect();
    // println!("{:?}", v2);

    // let max = v.iter().max();
    // println!("maximum:{:?}", max);

    // let min = v.iter().min();
    // println!("maximum:{:?}", max);

    // let sum: i32 = v.iter().sum();
    // let count = v.len() as i32;
    // let average = sum as i32 / count;
    // println!("Average: {}", average);

    let shared_v = Arc::new(v);

     let (tx, rx) = mpsc::channel();

    let max = max_value(v.clone());
    println!("Max: {}", max);

    let min = min_value(v.clone());
    println!("Min: {}", min);

    let avg = average(v.clone());
    println!("Average: {:.2}", avg);

    let med = median(v.clone());
    println!("Median: {:.2}", med);

    let v3 = vector_sum(v1.clone(), v2.clone());
    println!("{:?}", v3);
}
