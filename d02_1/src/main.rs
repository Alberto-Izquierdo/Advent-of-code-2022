use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    let file = args.iter().nth(1).unwrap();
    let lines = read_lines(file).unwrap();
    let total_score: i32 = lines
        .map(|line| {
            let line = line.unwrap();
            let mut chars = line.chars();
            let player_1_shape = char_to_shape(chars.next().unwrap());
            let last_character = chars.last().unwrap();
            let player_2_shape = get_next_shape(player_1_shape, last_character);
            get_score_from_shape(player_2_shape) + get_score_from_result(last_character)
        })
        .sum();
    println!("Score: {}", total_score);
}

// Transformation functions

fn shape_to_number(shape: Shape) -> i32 {
    match shape {
        Shape::ROCK => 0,
        Shape::PAPER => 1,
        Shape::SCISSORS => 2,
    }
}

fn number_to_shape(value: i32) -> Shape {
    match value {
        -1 => Shape::SCISSORS,
        0 => Shape::ROCK,
        1 => Shape::PAPER,
        2 => Shape::SCISSORS,
        3 => Shape::ROCK,
        _ => panic!(),
    }
}
fn char_to_shape(value: char) -> Shape {
    match value {
        'A' => Shape::ROCK,
        'B' => Shape::PAPER,
        'C' => Shape::SCISSORS,
        _ => panic!(),
    }
}

// Score functions

fn get_score_from_result(result: char) -> i32 {
    match result {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!(),
    }
}

fn get_score_from_shape(shape: Shape) -> i32 {
    match shape {
        Shape::ROCK => 1,
        Shape::PAPER => 2,
        Shape::SCISSORS => 3,
    }
}

fn get_next_shape(shape: Shape, direction: char) -> Shape {
    let number = shape_to_number(shape);
    let next_number = number
        + match direction {
            'X' => -1,
            'Y' => 0,
            'Z' => 1,
            _ => panic!(),
        };
    number_to_shape(next_number)
}

// IO functions

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
