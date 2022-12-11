use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

struct Directory {
    name: String,
    files: Vec<MyFile>,
    directories: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: &str) -> Rc<RefCell<Directory>> {
        Rc::new(RefCell::new(Directory {
            name: name.to_string(),
            files: vec![],
            directories: vec![],
            parent: None,
        }))
    }
    fn new_with_parent(name: &str, parent: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
        let new_dir = Rc::new(RefCell::new(Directory {
            name: name.to_string(),
            files: vec![],
            directories: vec![],
            parent: Some(parent.clone()),
        }));
        new_dir
    }
    fn get_child_directory(&self, name: &str) -> Rc<RefCell<Directory>> {
        if name == ".." {
            self.parent.clone().unwrap()
        } else {
            self.directories
                .iter()
                .find(|child| child.borrow().name == name)
                .unwrap()
                .clone()
        }
    }
    fn get_size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|file| file.size).sum();
        let directories_size: usize = self
            .directories
            .iter()
            .map(|directory| directory.borrow().get_size())
            .sum();
        files_size + directories_size
    }
    fn add_child_directory(&mut self, child: Rc<RefCell<Directory>>) {
        self.directories.push(child.clone())
    }
    fn _print(&self) {
        println!("{}", self.name);
        self.files.iter().for_each(|file| {
            println!("{} {}", file.size, file._name);
        });
        self.directories.iter().for_each(|directory| {
            (*directory).borrow()._print();
        });
    }
    fn get_directory_sizes(&self) -> Vec<usize> {
        let this_size = self.get_size();
        self.directories
            .iter()
            .flat_map(|directory| (*directory).borrow().get_directory_sizes())
            .chain(std::iter::once(this_size))
            .collect::<Vec<usize>>()
    }
}

struct MyFile {
    _name: String,
    size: usize,
}

impl MyFile {
    fn new(name: &str, size: usize) -> MyFile {
        MyFile {
            _name: name.to_string(),
            size,
        }
    }
}

fn main() {
    let file_name = get_nth_parameter(1);
    let lines = read_lines(&file_name).unwrap();
    let root = Directory::new("/");
    let mut current_dir = Rc::clone(&root);
    let _ = lines.skip(1).for_each(|line| {
        let line = line.unwrap();
        let mut words = line.split(' ');
        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();
        if first_word == "$" {
            // This is a command
            if second_word == "cd" {
                // We are only interested in "cd" commands to update the position
                let third_word = words.next().unwrap();
                let new_current_dir = current_dir.borrow().get_child_directory(third_word);
                current_dir = new_current_dir;
            }
        } else {
            // This is a result's command
            if first_word == "dir" {
                // This is a directory
                let new_dir = Directory::new_with_parent(second_word, current_dir.clone());
                (*current_dir).borrow_mut().add_child_directory(new_dir);
            } else {
                // This is a file
                (*current_dir).borrow_mut().files.push(MyFile::new(
                    second_word,
                    first_word.parse::<usize>().unwrap(),
                ));
            }
        }
    });
    let total_used_space = (*root).borrow().get_size();
    const TOTAL_SPACE: usize = 70000000;
    const UPDATE_SIZE: usize = 30000000;
    let free_space = TOTAL_SPACE - total_used_space;
    let space_to_free = UPDATE_SIZE - free_space;
    let directory_sizes = root.borrow().get_directory_sizes();
    let result: usize =
        directory_sizes
            .iter()
            .fold(TOTAL_SPACE, |current_directory_to_free, directory_size| {
                if directory_size >= &space_to_free && directory_size < &current_directory_to_free {
                    *directory_size
                } else {
                    current_directory_to_free
                }
            });
    println!("Result {}", result);
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
