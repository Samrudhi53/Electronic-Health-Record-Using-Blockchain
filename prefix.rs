use std::io;

fn main() {
    println!("Enter strings separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let strings: Vec<&str> = input.trim().split_whitespace().collect();

    let lcp = longest_common_prefix(strings);
    if lcp.is_empty() {
        println!("There is no common prefix.");
    } else {
        println!("The longest common prefix is: {}", lcp);
    }
}

fn longest_common_prefix(strings: Vec<&str>) -> String {
    if strings.is_empty() {
        return "".to_string();
    }

    let mut prefix = strings[0].to_string();

    for s in &strings[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return prefix;
            }
        }
    }

    prefix
}
