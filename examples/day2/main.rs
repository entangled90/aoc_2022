use aoc_2022::files::open_file;
use std::{error::Error, fs::*, io::Read};

#[derive(Debug, PartialEq, PartialOrd)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn from(char: &char) -> Option<RPS> {
        match char {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn round_points(&self, them: &RPS) -> u32 {
        if self == them {
            return 3;
        }
        match (self, them) {
            (RPS::Paper, RPS::Rock) => 6,
            (RPS::Scissors, RPS::Paper) => 6,
            (RPS::Rock, RPS::Scissors) => 6,
            _ => 0,
        }
    }
    pub fn choose_ours(&self, points_expected: u32) -> Option<RPS> {
        for us in vec![RPS::Rock, RPS::Paper, RPS::Scissors] {
            if us.round_points(self) == points_expected {
                return Some(us);
            }
        }
        return None;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let test_lines = vec!["A Y", "B X", "C Z"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("test: {}", solve_from_lines(&test_lines));

    let lines = open_file("examples/day2/input.txt".into())?;

    println!("Total points {}", solve_from_lines(&lines));
    return Ok(());
}

fn solve_from_lines(lines: &Vec<String>) -> u32 {
    let mut game: Vec<(RPS, RPS)> = Vec::with_capacity(1024);
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if let Some(opponent) = chars.get(0).and_then(RPS::from) {
            if let Some(us) = chars.get(2) {
                if let Some(us_rps) = match us {
                    'X' => opponent.choose_ours(0),
                    'Y' => opponent.choose_ours(3),
                    'Z' => opponent.choose_ours(6),
                    _ => None,
                } {
                    game.push((opponent, us_rps));
                }
            }
        }
    }
    let mut total_points: u32 = 0;
    for (them, us) in game {
        total_points += us.round_points(&them);
        total_points += us.points();
    }
    return total_points;
}
