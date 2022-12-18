use aoc_2022::files::open_file;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::ops::{Add, Deref};
use std::result::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(i32, i32);

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Point {
    fn norm(&self) -> usize {
        (self.0.abs() + self.1.abs()).try_into().unwrap()
    }
    fn distance(&self, other: &Point) -> usize {
        Point(other.0 - self.0, other.1 - self.1).norm()
    }

    fn frequency(&self) -> i64 {
        self.0 as i64 * 4000000 + self.1 as i64
    }

    fn point_inside(
        &self,
        distance: i32,
        delta_y: i32,
        min_x: i32,
        max_x: i32,
    ) -> impl Iterator<Item = Point> + '_ {
        (min_x..max_x).flat_map(move |i| {
            let delta = Point(i, delta_y);

            if (delta.norm() as i32) <= distance {
                let p = self + &Point(i, delta_y);
                // assert!(p.0 <= 20, "i= {}, {}", i, p.0);
                vec![p]
            } else {
                vec![]
            }
        })
    }
}

#[derive(Debug)]
pub struct Sensor(Point);

impl Deref for Sensor {
    type Target = Point;

    fn deref(&self) -> &Point {
        &self.0
    }
}

#[derive(Debug)]
pub struct Beacon(Point);

impl Deref for Beacon {
    type Target = Point;

    fn deref(&self) -> &Point {
        &self.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = open_file("examples/day15/input.txt").expect("Failed to open file");
    let world = parse(&lines)?;
    for (s, b) in &world {
        println!("{:?} -> {:?}", s, b);
    }

    let all_points: Vec<Point> = world.iter().flat_map(|(s, b)| vec![s.0, b.0]).collect();
    let min_x = all_points.iter().map(|p| p.0).min().ok_or("no min x")?;
    let max_x = all_points.iter().map(|p| p.0).max().ok_or("no max x")?;
    let min_y = all_points.iter().map(|p| p.1).min().ok_or("no min y")?;
    let max_y = all_points.iter().map(|p| p.1).max().ok_or("no max y")?;
    println!("World size ({},{}), ({},{})", min_x, max_x, min_y, max_y);

    // let mut removed_points: HashSet<Point> = HashSet::new();

    // let row_y = 2000000;

    // for (s, b) in &world {
    //     let distance = s.distance(&b);
    //     if row_y < (s.0 .1 + distance as i32) && row_y > (s.0 .1 - distance as i32) {
    //         println!("Generating points for S={:?} B={:?}: {}", s, b, distance);
    //         let delta_y = row_y - s.0 .1;
    //         let points = s.point_inside(
    //             distance as i32,
    //             delta_y,
    //             -1 * distance as i32,
    //             distance as i32,
    //         );
    //         for p in points {
    //             removed_points.insert(p);
    //         }
    //     }
    // }
    // println!("Points generated");
    // // remove all beacons

    // for (s, b) in &world {
    //     removed_points.remove(b);
    //     removed_points.remove(s);
    // }

    // let count: usize = removed_points.len();

    // println!("Count is {}", count);

    // if row_y == 10 {
    //     let mut xs = removed_points
    //         .iter()
    //         .map(|Point(x, y)| x)
    //         .collect::<Vec<_>>();
    //     xs.sort();
    // }

    ///// ------------------------------------------------------------------------------------------------
    // part 2
    ///// ------------------------------------------------------------------------------------------------

    println!("PART 2");
    let max_beacon = 4000000;
    // let max_beacon = 20;
    // let mut removed_points_2: HashSet<Point> = HashSet::new();

    // for (s, b) in &world {
    //     let distance = s.distance(&b);
    //     println!("Generating points for S={:?} B={:?}: {}", s, b, distance);

    //     let min_y = std::cmp::max(0, s.0 .1 - distance as i32);
    //     let max_y = std::cmp::min(max_beacon, s.0 .1 + distance as i32);

    //     let min_x = std::cmp::max(0, s.0 .0 - distance as i32);
    //     let max_x = std::cmp::min(max_beacon, s.0 .0 + distance as i32);
    //     println!("{:?}, {:?}", (min_x, max_x), (min_y, max_y));

    //     let min_delta_x = min_x - s.0 .0;
    //     let max_delta_x = max_x - s.0 .0;
    //     let min_delta_y = min_y - s.1;
    //     let max_delta_y = max_y - s.1;

    //     println!(
    //         "x: {:?}, y:{:?}",
    //         (min_delta_x, max_delta_x),
    //         (min_delta_y, max_delta_y)
    //     );
    //     //TODO
    //     // limit the search space in the rectangle
    //     for j in min_delta_y..max_delta_y + 1 {
    //         for p in s.point_inside(distance as i32, j, min_delta_x, max_delta_x) {
    //             // println!("Point {:?} excluded", p);
    //             removed_points_2.insert(p);
    //         }
    //     }
    // }
    // for i in 0..max_beacon {
    //     for j in 0..max_beacon {
    //         let p = Point(i, j);
    //         if !removed_points_2.contains(&p) {
    //             println!("{:?} not in map: score {}", &p, p.0 * 4000000 + p.1);
    //         }
    //     }
    // }
    // println!("Count {}", removed_points_2.len());

    let _res = (0..max_beacon).into_par_iter().try_for_each(|i| {
        let mut j: usize = 0;
        if i % 100 == 0 {
            println!("running iteration {}", i);
        }
        while j < max_beacon {
            let mut close_to_someone = false;
            let p = Point(i as i32, j as i32);

            for (s, b) in &world {
                let distance = s.distance(&b);
                let distance_to_sensor = p.distance(&s);
                if distance_to_sensor <= distance {
                    j += max(1, (distance - distance_to_sensor)) as usize;
                    // println!("Found match, skipping {}", distance - distance_to_sensor);
                    close_to_someone = true;
                    break;
                }
            }
            if !close_to_someone {
                println!("Point {:?} is not close to anyone! {}", p, p.frequency());
                return Err(());
            }
        }
        Ok(())
    });
    Ok(())
}

fn parse(lines: &[String]) -> Result<Vec<(Sensor, Beacon)>, Box<dyn Error>> {
    let regex = Regex::new(
        r"Sensor at x=(\-*\d+), y=(\-*\d+): closest beacon is at x=(\-*\d+), y=(\-*\d+)",
    )?;

    let mut res = Vec::new();
    for l in lines {
        let capt = regex
            .captures(&l)
            .ok_or("Failed to get caputres".to_string())?;
        let s_x: i32 = capt.get(1).unwrap().as_str().parse()?;
        let s_y: i32 = capt.get(2).unwrap().as_str().parse()?;
        let b_x: i32 = capt.get(3).unwrap().as_str().parse()?;
        let b_y: i32 = capt.get(4).unwrap().as_str().parse()?;
        res.push((Sensor(Point(s_x, s_y)), Beacon(Point(b_x, b_y))));
    }
    Ok(res)
}
