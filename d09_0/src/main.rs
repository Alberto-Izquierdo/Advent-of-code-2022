use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = get_nth_parameter(1);
    let _lines = read_lines(&file_name).unwrap();
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut tail_positions_set: HashSet<(i32, i32)> = HashSet::new();
    tail_positions_set.insert(tail_position);
    _lines.for_each(|line| {
        let line = line.unwrap();
        let mut motions = line.split(' ');
        let direction = motions.next().unwrap();
        let distance = motions.next().unwrap().parse::<i32>().unwrap();
        let direction_vector = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..distance {
            head_position.0 += direction_vector.0;
            head_position.1 += direction_vector.1;
            if !are_positions_adjacent(head_position, tail_position) {
                tail_position = (
                    head_position.0 - direction_vector.0,
                    head_position.1 - direction_vector.1,
                );
                tail_positions_set.insert(tail_position);
            }
        }
    });
    let result = tail_positions_set.len();
    println!("Result: {}", result);
}

fn are_positions_adjacent(head_position: (i32, i32), tail_position: (i32, i32)) -> bool {
    let distance_x = (head_position.0 - tail_position.0).abs();
    let distance_y = (head_position.1 - tail_position.1).abs();
    distance_x <= 1 && distance_y <= 1
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
