use aoc_2022::files::open_file;
use std::{error::Error, fs::*, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let contents = open_file("examples/day1/input.txt".into())?;
    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut current_elph: Vec<i32> = Vec::new();
    for line in contents {
        if !line.is_empty() {
            let value: i32 = line.parse()?;
            current_elph.push(value);
        } else {
            let finished = current_elph;
            current_elph = Vec::new();
            elves.push(finished)
        }
    }
    // for (i, elph) in elves.iter().enumerate() {
    //     println!("Elph {}: {:?}", i, elph);
    // }
    let mut result: Vec<i32> = elves
        .iter()
        // .enumerate()
        .map(|elph| elph.iter().sum::<i32>())
        // .enumerate()
        .collect();
    result.sort();
    result.reverse();
    let first_three: Vec<&i32> = result.iter().take(3).collect();
    println!("First 3: {:?}", first_three);
    println!("Result is {:?}", result.iter().take(3).sum::<i32>());
    return Ok(());
}
