use std::io;
use std::cmp;

fn main() {
    println!("Enter an array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let max_sum = max_subarray_sum(&numbers);
    println!("The maximum subarray sum is: {}", max_sum);
}

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut current_sum = 0;
    let mut max_sum = i32::MIN;

    for &num in nums {
        current_sum = cmp::max(num, current_sum + num);
        max_sum = cmp::max(max_sum, current_sum);
    }

    max_sum
}
