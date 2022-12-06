use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    let file = args.iter().nth(1).unwrap();
    let lines = read_lines(file).unwrap();
    let total_score: u32 = lines
        .map(|line| {
            let line = line.unwrap();
            let mut chars = line.chars();
            let first_transformed = std::char::from_u32(chars.next().unwrap() as u32 + 23).unwrap();
            let last = chars.last().unwrap();
            get_score(first_transformed, last)
        })
        .sum();
    println!("Score: {}", total_score);
}

fn get_score(first: char, second: char) -> u32 {
    get_sign_score(second) + compare_signs(first, second)
}

fn get_sign_score(value: char) -> u32 {
    match value {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(),
    }
}

fn compare_signs(first: char, second: char) -> u32 {
    if first == second {
        3
    } else {
        if first == 'X' && second == 'Y' {
            6
        } else if first == 'Y' && second == 'Z' {
            6
        } else if first == 'Z' && second == 'X' {
            6
        } else {
            0
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
