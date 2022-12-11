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
    let mut visible_trees = 0;
    for y in 1..forest.len() - 1 {
        for x in 1..forest[0].len() - 1 {
            if is_tree_visible(&forest, x as usize, y as usize) {
                visible_trees += 1;
            }
        }
    }
    // Add edges
    visible_trees += forest.len() * 2;
    visible_trees += forest[0].len() * 2;
    // Remove duplicated corners
    visible_trees -= 4;
    println!("Result: {}", visible_trees);
}

fn is_tree_visible(forest: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let current_height = &forest[y][x];
    // left
    let mut left_visible = true;
    for left in 0..x {
        if &forest[y][left] >= current_height {
            left_visible = false;
            break;
        }
    }
    if left_visible {
        return true;
    }
    // right
    let mut right_visible = true;
    for right in (x + 1)..forest[y].len() {
        if &forest[y][right] >= current_height {
            right_visible = false;
            break;
        }
    }
    if right_visible {
        return true;
    }
    // top
    let mut top_visible = true;
    for top in 0..y {
        if &forest[top][x] >= current_height {
            top_visible = false;
            break;
        }
    }
    if top_visible {
        return true;
    }
    // bottom
    let mut bottom_visible = true;
    for bottom in (y + 1)..forest.len() {
        if &forest[bottom][x] >= current_height {
            bottom_visible = false;
            break;
        }
    }
    if bottom_visible {
        return true;
    }
    false
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
