use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ElfPair {
    first_elf: u128,
    second_elf: u128,
}

struct Assignment {
    first_section: u128,
    last_section: u128,
}

fn main() {
    let file_name = get_nth_parameter(1);
    let elves_information = parse_file(&file_name);
    let result = get_elves_overlaping(&elves_information);
    println!("Result: {}", result);
}

fn get_elves_overlaping(info: &Vec<ElfPair>) -> usize {
    info.iter()
        .filter(|elf_pair| do_elves_information_overlap(*elf_pair))
        .count()
}

fn do_elves_information_overlap(pair: &ElfPair) -> bool {
    let or_result = pair.first_elf | pair.second_elf;
    pair.first_elf == or_result || pair.second_elf == or_result
}

fn parse_file(file_name: &str) -> Vec<ElfPair> {
    let lines = read_lines(file_name).unwrap();
    // Transform lines to ranges
    lines
        .map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|pair| pair.to_string())
                .collect::<Vec<String>>()
        })
        .map(|string| {
            assert!(string.len() == 2);
            let mut assignments = string.iter().map(|assignment_str| {
                let mut iter = assignment_str
                    .split('-')
                    .map(|number| number.to_string().parse::<u128>().unwrap());
                let first_section = iter.next().unwrap();
                let last_section = iter.next().unwrap();
                let assignment = Assignment {
                    first_section,
                    last_section,
                };
                assignment_to_bits(assignment)
            });
            let first_elf = assignments.next().unwrap();
            let second_elf = assignments.next().unwrap();
            ElfPair {
                first_elf,
                second_elf,
            }
        })
        .collect::<Vec<ElfPair>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    #[test]
    fn test_function() {
        assert_eq!(
            assignment_to_bits(Assignment {
                first_section: 1,
                last_section: 2
            }),
            6
        );
        assert_eq!(
            assignment_to_bits(Assignment {
                first_section: 1,
                last_section: 3
            }),
            14
        );
        assert_eq!(
            assignment_to_bits(Assignment {
                first_section: 2,
                last_section: 3
            }),
            12
        );
    }
}

fn assignment_to_bits(range: Assignment) -> u128 {
    assert!(range.first_section <= range.last_section);
    let mut result: u128 = 0;
    for i in range.first_section..(range.last_section + 1) {
        result |= (2 as u128).pow(i as u32);
    }
    result
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
