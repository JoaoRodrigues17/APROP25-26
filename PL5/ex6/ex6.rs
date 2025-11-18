
//a) sum of all numbers from 1 to 100 using iterators
fn sum_1_to_100()-> i32{
    (1..=100).sum()
}

//b) sum of all squares lower than a “upper_bound” value

fn sum_all_squares_lower_than(x:i32)->i32{
    let last_n = ((x - 1)as f64).sqrt() as i32;
    (1..=last_n).map(|n| n*n).sum()
}

fn main(){
    println!("Sum of values from 1 to 100: {:?}", sum_1_to_100());
    println!("Sum of squares lower than 100 (sum of x^2 from 1 to x where x^2 < 100): {:?}",sum_all_squares_lower_than(100))
}