use std::io;

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let median = find_median(&numbers);
    println!("The median is: {}", median);
}

fn find_median(numbers: &[i32]) -> f32 {
    let len = numbers.len();
    if len == 0 {
        panic!("Array is empty");
    }
    if len % 2 == 1 {
        numbers[len / 2] as f32
    } else {
        (numbers[len / 2 - 1] as f32 + numbers[len / 2] as f32) / 2.0
    }
}
