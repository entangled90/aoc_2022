use std::{collections::HashSet, fs::*};

pub fn main() {
    println!(
        "Test: {}",
        find_distinct_chars("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)
    );

    let input = read_to_string("examples/day6/input.txt").expect("Missing file");
    println!("Solution One: {}", find_distinct_chars(&input, 4));

    println!("Solution Two: {}", find_distinct_chars(&input, 14));
}

fn find_distinct_chars(s: &str, n: usize) -> usize {
    for i in 0..(s.len() - n) {
        let slice = &s[i..i + n];
        if slice.chars().collect::<HashSet<_>>().len() == n {
            return i + n;
        }
    }
    return 0;
}
