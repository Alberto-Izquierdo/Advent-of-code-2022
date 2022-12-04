use std::collections::BTreeSet;
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
    let values = lines.fold(vec![0], |mut acc, line| {
        let line = line.unwrap();
        if line.is_empty() {
            acc.push(0);
        } else {
            *acc.last_mut().unwrap() += line.parse::<i32>().unwrap();
        }
        acc
    });
    let set = values.iter().fold(BTreeSet::new(), |mut set, value| {
        set.insert(value);
        set
    });
    let max_3_values: i32 = set.into_iter().rev().take(3).map(|v| *v).sum();
    println!("Sum of 3 max values: {:?}", max_3_values);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
