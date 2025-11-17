fn sum_1_to_100() -> i32 {
    (1..=100).sum()
}

fn sum_of_squares_below(upper_bound: u32) -> u32 {
    (1..)
        .map(|x| x * x)
        .take_while(|&sq| sq < upper_bound)
        .sum()
}
fn main() {
    let result = sum_1_to_100();
    println!("The sum is: {}", result);

    let result = sum_of_squares_below(100);
    println!("Sum of squares below 100: {}", result);
}
