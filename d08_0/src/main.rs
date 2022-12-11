use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = get_nth_parameter(1);
    let lines = read_lines(&file_name).unwrap();
    let forest = lines
        .into_iter()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|character| character.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut highest_scenic_score = 0;
    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            let current_scenic_score = get_scenic_score(&forest, x as usize, y as usize);
            if current_scenic_score > highest_scenic_score {
                highest_scenic_score = current_scenic_score;
            }
        }
    }
    println!("Result: {}", highest_scenic_score);
}

fn get_scenic_score(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let current_height = &forest[y][x];
    // left
    let mut left_seen_trees = 0;
    for left in (0..x).rev() {
        left_seen_trees += 1;
        if &forest[y][left] >= current_height {
            break;
        }
    }
    // right
    let mut right_seen_trees = 0;
    for right in (x + 1)..forest[y].len() {
        right_seen_trees += 1;
        if &forest[y][right] >= current_height {
            break;
        }
    }
    // top
    let mut top_seen_trees = 0;
    for top in (0..y).rev() {
        top_seen_trees += 1;
        if &forest[top][x] >= current_height {
            break;
        }
    }
    // bottom
    let mut bottom_seen_trees = 0;
    for bottom in (y + 1)..forest.len() {
        bottom_seen_trees += 1;
        if &forest[bottom][x] >= current_height {
            break;
        }
    }
    left_seen_trees * right_seen_trees * top_seen_trees * bottom_seen_trees
}

fn get_nth_parameter(index: usize) -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    args.iter().nth(index).unwrap().to_string()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
