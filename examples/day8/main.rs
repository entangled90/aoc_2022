use aoc_2022::files::open_file;

pub fn main() {
    let lines = open_file("examples/day8/input.txt").expect("Failed to open file");
    let trees: Trees = Trees::new(lines);
    let directions: Vec<(i8, i8)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut count = 0;
    for i in 0..trees.matrix.len() {
        for j in 0..trees.matrix[0].len() {
            if let Some(_) = directions
                .iter()
                .find(|dir| trees.visible_from((i, j), **dir))
            {
                count += 1;
            }
        }
    }
    println!("Count is {}", count);

    let mut max_score = 0;
    for i in 0..trees.matrix.len() {
        for j in 0..trees.matrix[0].len() {
            let scenic_score: usize = directions
                .iter()
                .map(|dir| {
                    let score = trees.visible_trees((i, j), *dir);
                    score
                })
                .product();

            if scenic_score > max_score {
                max_score = scenic_score;
            }
        }
    }
    println!("Max scenic score is {}", max_score);
}

pub struct Trees {
    matrix: Vec<Vec<u32>>,
}

impl Trees {
    pub fn new(lines: Vec<String>) -> Self {
        Trees {
            matrix: lines
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).expect("failed to convert height"))
                        .collect()
                })
                .collect(),
        }
    }

    pub fn visible_from(&self, tree: (usize, usize), direction: (i8, i8)) -> bool {
        let (x, y) = tree;
        let height = self.matrix[x][y];
        let (mut dir_x, mut dir_y) = direction;
        let mut visible = true;
        while visible {
            let new_x: i8 = x as i8 + dir_x;
            let new_y: i8 = y as i8 + dir_y;
            // println!("direction is now {:?}", (new_x, new_y));
            if new_x < 0 || new_y < 0 {
                // println!("< 0 with direction {:?}", direction);
                break;
            }
            let taller = self
                .matrix
                .get(new_x as usize)
                .and_then(|v| v.get(new_y as usize));
            match taller {
                Some(other_height) => visible = visible && (*other_height < height),
                None => {
                    break;
                }
            }
            dir_x += direction.0;
            dir_y += direction.1;
        }
        visible
    }

    pub fn visible_trees(&self, tree: (usize, usize), direction: (i8, i8)) -> usize {
        let (x, y) = tree;
        let height = self.matrix[x][y];
        let (mut dir_x, mut dir_y) = direction;
        let mut visible = 0;
        loop {
            let new_x: i8 = x as i8 + dir_x;
            let new_y: i8 = y as i8 + dir_y;
            // println!("direction is now {:?}", (new_x, new_y));
            if new_x < 0 || new_y < 0 {
                // println!("< 0 with direction {:?}", direction);
                break;
            }
            let taller = self
                .matrix
                .get(new_x as usize)
                .and_then(|v| v.get(new_y as usize));
            match taller {
                Some(other_height) if *other_height < height => visible += 1,
                Some(t) => {
                    visible += 1;
                    break;
                }
                None => break,
            }
            dir_x += direction.0;
            dir_y += direction.1;
        }
        visible
    }
}
