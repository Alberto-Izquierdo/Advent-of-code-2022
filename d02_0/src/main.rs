use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("An input file is required");
    }
    let file = args.iter().nth(1).unwrap();
    let lines = read_lines(file).unwrap();
    // Consumes the iterator, returns an (Optional) String
    let result: u32 = lines
        .map(|line| {
            let line = line.unwrap();
            process_line(&line)
        })
        .sum();
    println!("Result: {}", result);
}

fn process_line(line: &str) -> u32 {
    let (first, second) = line.split_at(line.len() / 2);
    let set = first.chars().collect::<HashSet<char>>();
    let common_value = second
        .chars()
        .find(|value| set.get(value).is_some())
        .unwrap();
    get_character_value(common_value)
}

fn get_character_value(character: char) -> u32 {
    if character >= 'A' && character <= 'Z' {
        character as u32 - 38
    } else {
        character as u32 - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[test]
    fn test_p() {
        assert_eq!(get_character_value('p'), 16);
    }

    #[test]
    fn test_upper_l() {
        assert_eq!(get_character_value('L'), 38);
    }

    #[test]
    fn test_upper_p() {
        assert_eq!(get_character_value('P'), 42);
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
