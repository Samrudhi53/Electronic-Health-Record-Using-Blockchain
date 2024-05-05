fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace()
     .min_by_key(|word| word.len())
}

fn main() {
    let input_string = "Find the shortest word in this sentence";
    match shortest_word(input_string) {
        Some(word) => println!("The shortest word is: '{}'", word),
        None => println!("No words found"),
    }
}
