use std::io;

fn main() {
    println!("Enter an array of integers separated by spaces:");
    let mut array_input = String::new();
    io::stdin()
        .read_line(&mut array_input)
        .expect("Failed to read line");

    let mut array: Vec<i32> = array_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin()
        .read_line(&mut k_input)
        .expect("Failed to read line");

    let k: usize = k_input.trim().parse().expect("Please enter a valid number");

    if k == 0 || k > array.len() {
        println!("k is out of the valid range (1 to {})", array.len());
        return;
    }

    array.sort(); // Sorting the array
    let kth_smallest = array[k - 1]; // kth smallest element, considering k is 1-based index
    println!("The {}th smallest element is {}", k, kth_smallest);
}

