use aoc_2022::files::open_file;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::*,
    io::Read,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let lines = open_file("examples/day3/input.txt")?;

    let mut prios = Vec::new();
    for line in &lines {
        let (first, second) = line.split_at(line.len() / 2);
        std::assert!(first.len() == second.len());
        let first_set: HashSet<char> = HashSet::from_iter(first.chars());
        let second_set: HashSet<char> = HashSet::from_iter(second.chars());
        let intersection = first_set.intersection(&second_set);
        for inter in intersection {
            let p = priority(inter).unwrap();
            println!("In inter {}: {}", inter, p);

            prios.push(p);
        }
    }
    println!("First solution {}", prios.iter().sum::<u32>());

    let mut group = Vec::with_capacity(3);
    let mut groups = Vec::new();
    for line in &lines {
        group.push(line);
        if group.len() == 3 {
            groups.push(group.clone());
            group.clear()
        }
    }

    let mut labels = Vec::new();
    for group in groups {
        let intersection = group
            .iter()
            .map(|l| HashSet::from_iter(l.chars()))
            .reduce(|acc: HashSet<char>, item| {
                HashSet::from_iter(acc.intersection(&item).map(|c| c.clone()))
            })
            .expect("No intersection");
        labels.push(intersection.into_iter().next().unwrap());
    }
    println!(
        "Second solution : {}",
        labels.iter().map(|c| priority(c).unwrap()).sum::<u32>()
    );
    Ok(())
}

fn priority(c: &char) -> Result<u32, Box<dyn Error>> {
    let ascii = *c as u32;
    if ascii >= 97 {
        return Ok(ascii - 97 + 1);
    } else if ascii >= 65 {
        return Ok(ascii - 65 + 27);
    } else {
        return Err("invalid char".into());
    }
}
