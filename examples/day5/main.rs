use std::{cmp::min, collections::VecDeque};

use aoc_2022::files::open_file;

pub fn main() {
    let lines = open_file("examples/day5/input.txt").expect("Failed to open file");
    let mut header_done = false;
    let mut starting_config: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    let mut instructions: Vec<(u32, usize, usize)> = Default::default();

    for line in lines {
        if !header_done {
            if line.trim().len() == 0 {
                header_done = true;
            } else {
                for i in 0..(line.len() / 4 + 1) {
                    let end = min((i + 1) * 4, line.len());
                    let slice = &line[i * 4..end].trim();
                    if slice.starts_with('[') {
                        let id = slice.chars().collect::<Vec<_>>()[1];
                        starting_config[i].push_back(id);
                    }
                }
            }
        } else {
            let words: Vec<_> = line.split(' ').collect();
            match &words[..] {
                ["move", n_str, "from", from_str, "to", to_str] => {
                    let n = n_str.parse().unwrap();
                    let from: usize = from_str.parse::<usize>().unwrap() - 1;
                    let to: usize = to_str.parse::<usize>().unwrap() - 1;
                    instructions.push((n, from, to));
                }
                v => panic!("Invalid instruction {:?}", v),
            }
        }
    }

    // get the top of the stacks:

    println!(
        "Solution one {}",
        solution_one(&starting_config, &instructions)
    );
    println!(
        "Solution two {}",
        solution_two(&starting_config, &instructions)
    );
}

fn solution_one(config: &Vec<VecDeque<char>>, instructions: &[(u32, usize, usize)]) -> String {
    let mut state = config.clone();

    for (n, from, to) in instructions {
        for _ in 0..*n {
            if let Some(el) = state.get_mut(*from).expect("Missing from slot").pop_front() {
                state.get_mut(*to).expect("Missing to slot").push_front(el)
            }
        }
    }
    return top_of_stack(&state);
}

fn solution_two(config: &Vec<VecDeque<char>>, instructions: &[(u32, usize, usize)]) -> String {
    let mut state = config.clone();

    for (n, from, to) in instructions {
        let mut tmp = Vec::new();
        for _ in 0..*n {
            if let Some(el) = state.get_mut(*from).expect("Missing from slot").pop_front() {
                tmp.push(el);
            }
        }
        tmp.reverse();
        for el in tmp {
            state.get_mut(*to).expect("Missing to slot").push_front(el);
        }
    }
    return top_of_stack(&state);
}

fn top_of_stack(cfg: &Vec<VecDeque<char>>) -> String {
    return cfg
        .iter()
        .map(|dq| dq.get(0).expect("missing top of stack"))
        .collect();
}
