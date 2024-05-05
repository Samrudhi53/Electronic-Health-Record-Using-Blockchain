use std::io;

fn is_palindrome(s: &str) -> bool {
    let filtered_chars: Vec<char> = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let reversed_chars = filtered_chars.iter().rev().cloned().collect::<Vec<char>>();
    filtered_chars == reversed_chars
}

fn main() {
    let mut input_string = String::new();
    println!("Please enter a string to check if it is a palindrome:");
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    // Trim the newline character from the input
    input_string = input_string.trim_end().to_string();

    println!("Is '{}' a palindrome? {}", input_string, is_palindrome(&input_string));
}
