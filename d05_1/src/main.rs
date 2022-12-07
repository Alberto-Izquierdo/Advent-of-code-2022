use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = get_nth_parameter(1);
    let lines = read_lines(&file_name).unwrap();
    let mut lines_iter = lines.map(|line| line.unwrap());
    let mut stacks = lines_iter
        .by_ref()
        .take_while(|line| line.chars().nth(1).unwrap() != '1')
        .fold(Vec::new(), |mut stacks: Vec<Vec<char>>, line| {
            if stacks.is_empty() {
                stacks.resize((line.len() / 4) + 1, Vec::new());
            }
            for index in (0..(line.len() / 4) + 1).step_by(1) {
                let char_position = 1 + index * 4;
                let current_crate = line.chars().nth(char_position).unwrap();
                if current_crate != ' ' {
                    stacks.get_mut(index).unwrap().insert(0, current_crate);
                }
            }
            stacks
        });
    lines_iter.skip(1).for_each(|line| {
        let mut line_divided = line.split(' ');
        let quantity_to_move = line_divided
            .by_ref()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let origin = line_divided
            .by_ref()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;
        let destiny = line_divided
            .by_ref()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;
        let mut crates_to_move = String::new();
        for _ in 0..quantity_to_move {
            let current_crate = stacks.get_mut(origin).unwrap().pop().unwrap();
            crates_to_move.push(current_crate);
        }
        crates_to_move.chars().rev().for_each(|crate_to_move| {
            stacks.get_mut(destiny).unwrap().push(crate_to_move);
        });
    });
    let result = stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>();
    println!("Result: {}", result);
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
