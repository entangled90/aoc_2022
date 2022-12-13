use aoc_2022::files::open_file;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::*,
    io::Read,
    ops::Range,
};

pub fn main() {
    let ass = solution_one();
    solution_two(&ass);
}

fn solution_one() -> Vec<(Assignment, Assignment)> {
    let assignments = parse_file("examples/day4/input.txt");
    println!("Assignments {:?}", assignments);
    let overlaps = assignments
        .iter()
        .filter(|(a1, a2)| a1.overlaps_completely(a2))
        .count();
    println!("Solution 1 {}", overlaps);
    return assignments;
}

fn solution_two(assignments: &[(Assignment, Assignment)]) {
    let overlaps = assignments.iter().filter(|(a1, a2)| a1.overlap(a2)).count();
    println!("Solution 2 {:?}", overlaps);
}

#[derive(Debug)]
struct Assignment(u32, u32);

impl Assignment {
    fn overlaps_completely(&self, other: &Assignment) -> bool {
        return self.contains(other) || other.contains(self);
    }

    fn overlap(&self, other: &Assignment) -> bool {
        return (self.1 >= other.0 && self.0 <= other.1)
            || (other.1 >= self.0 && other.0 <= self.1);
    }

    fn contains(&self, other: &Assignment) -> bool {
        return self.0 <= other.0 && self.1 >= other.1;
    }
}

fn parse_file(path: &str) -> Vec<(Assignment, Assignment)> {
    let lines = open_file(path).expect("Can't open file");

    let mut assignments = Vec::new();
    for line in lines {
        let pairs: Vec<&str> = line.split(',').collect();
        match &pairs[..] {
            [a1, a2] => {
                let range1: Vec<u32> = a1.split('-').map(|s| s.parse().unwrap()).collect();
                let range2: Vec<u32> = a2.split('-').map(|s| s.parse().unwrap()).collect();
                assignments.push((
                    Assignment(range1[0], range1[1]),
                    Assignment(range2[0], range2[1]),
                ));
            }
            _ => panic!("Wrong parsing"),
        }
    }

    assignments
}
