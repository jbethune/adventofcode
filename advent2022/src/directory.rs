use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Directory {
    files: HashMap<String, usize>,
    directories: HashMap<String, Directory>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, name: String, size: usize) {
        self.files.insert(name, size);
    }

    pub fn add_directory(&mut self, name: String) {
        self.directories.entry(name).or_insert(Directory::new());
    }

    pub fn total_size(&self) -> usize {
        let mut result = 0;
        for size in self.files.values() {
            result += size;
        }

        for directory in self.directories.values() {
            result += directory.total_size();
        }

        result
    }

    pub fn descend(&mut self, dir_name: &str) -> &mut Directory {
        self.directories.get_mut(dir_name).unwrap()
    }

    pub fn subdirs(&self) -> std::collections::hash_map::Values<String, Directory> {
        self.directories.values()
    }
}

pub fn walk_through_commands_from_file(path: &str) -> Directory {
    let mut result = Directory::new();
    let fd = File::open(path).unwrap();
    let reader = BufReader::new(fd);

    let mut current_path: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();
        match tokens[0].as_str() {
            "$" => {
                match tokens[1].as_str() {
                    "cd" => match tokens[2].as_str() {
                        "/" => current_path.clear(),
                        ".." => {
                            current_path.pop();
                        }
                        name => current_path.push(name.to_string()),
                    },
                    "ls" => {
                        // just parse the following lines
                    }
                    _ => {
                        panic!("unsupported command: {}", tokens[1])
                    }
                }
            }
            "dir" => {
                let mut dir_pointer = &mut result;
                for dir_name in &current_path {
                    dir_pointer = dir_pointer.descend(&dir_name);
                }
                dir_pointer.add_directory(tokens[1].to_string());
            }
            size => {
                let mut dir_pointer = &mut result;
                for dir_name in &current_path {
                    dir_pointer = dir_pointer.descend(&dir_name);
                }
                dir_pointer.add_file(tokens[1].to_string(), size.parse().unwrap());
            }
        }
    }
    result
}

pub fn traverse_directories_and_gather_sizes(dir: &Directory) -> Vec<usize> {
    let mut result = vec![dir.total_size()];
    for subdir in dir.subdirs() {
        result.extend(traverse_directories_and_gather_sizes(&subdir));
    }
    result
}
