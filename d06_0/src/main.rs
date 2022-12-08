use std::collections::{HashSet, LinkedList};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_name = get_nth_parameter(1);
    let stream = read_lines(&file_name).unwrap().next().unwrap().unwrap();
    let mut list: LinkedList<char> = LinkedList::new();
    let mut stream_iterator = stream.chars();
    let list_size = 4;
    stream_iterator
        .by_ref()
        .take(list_size)
        .for_each(|character| list.push_back(character));
    let position = stream_iterator
        .take_while(|character| {
            let set = list
                .iter()
                .map(|character| *character)
                .collect::<HashSet<char>>();
            let found = list.len() == set.len();
            list.pop_front();
            list.push_back(*character);
            !found
        })
        .count()
        + list_size;
    println!("Result: {}", position);
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
