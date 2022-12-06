use std::collections::HashSet;
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
    let mut lines = read_lines(file).unwrap();
    let mut next = lines.next();
    let mut result = 0;
    while next.is_some() {
        let set1 = next.unwrap().unwrap().chars().collect::<HashSet<char>>();
        let set2 = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect::<HashSet<char>>();
        let common_char = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .find(|value| set1.contains(value) && set2.contains(value));
        result += get_character_value(common_char.unwrap());
        next = lines.next();
    }
    println!("Result: {}", result);
}

fn get_character_value(character: char) -> u32 {
    if character >= 'A' && character <= 'Z' {
        character as u32 - 38
    } else {
        character as u32 - 96
    }
}

// IO

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
