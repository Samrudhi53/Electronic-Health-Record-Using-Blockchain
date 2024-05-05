use std::io;

fn main() {
    println!("Enter the first sorted array, separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter the second sorted array, separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let array1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();
    let array2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let merged_array = merge_sorted_arrays(&array1, &array2);
    println!("Merged array: {:?}", merged_array);
}

fn merge_sorted_arrays(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(array1.len() + array2.len());
    let mut i = 0;
    let mut j = 0;

    while i < array1.len() && j < array2.len() {
        if array1[i] <= array2[j] {
            result.push(array1[i]);
            i += 1;
        } else {
            result.push(array2[j]);
            j += 1;
        }
    }

    while i < array1.len() {
        result.push(array1[i]);
        i += 1;
    }

    while j < array2.len() {
        result.push(array2[j]);
        j += 1;
    }

    result
}
