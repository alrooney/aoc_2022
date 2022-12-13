use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use regex::Regex;

#[derive(Debug)]
struct File {
    name: String,
    size: i64,
}

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<Box<File>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn find_dir(&self, name: &str ) -> Option<Rc<RefCell<Dir>>> {
        self.dirs.iter().find_map(|t_dir| {
            if t_dir.borrow().name == name {
                Some(Rc::clone(&t_dir))
            }
            else {
                None
            }
        })
    }

    fn get_size(&self) -> i64 {
        self.dirs.iter().map(|dir| dir.borrow().get_size()).collect::<Vec<i64>>().iter().sum::<i64>() +
            self.files.iter().map(|file| file.size).collect::<Vec<i64>>().iter().sum::<i64>()
    }

    fn all_dirs(&self, dirs: &mut Vec<Rc<RefCell<Dir>>>) {
        self.dirs.iter().for_each(|dir| {
            dir.borrow().all_dirs(dirs);
            dirs.push(Rc::clone(&dir));
        });
    }
}

fn main() {
    // let file_path = "terminal_output_sample.txt";
    let file_path = "terminal_output.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");
    let file_system = Rc::new(RefCell::new(Dir {name: "/".to_string(), files: vec![], dirs: vec![], parent: None}));
    let mut current_dir = Rc::clone(&file_system);
    let line_re = Regex::new(r"^(\$\s\w\w\s?.*)?(dir\s.+)?(\d+\s.+)?$").unwrap();
    let command_re = Regex::new(r"^\$\s(cd)?(ls)?\s?(.+)?$").unwrap();
    let directory_re = Regex::new(r"^dir\s(.+)$").unwrap();
    let file_re = Regex::new(r"^(\d+)\s(.+)$").unwrap();
    for line in contents.lines() {
        dbg!(line);
        if let Some(line_cap) = line_re.captures(line) {
            if let Some(command) = line_cap.get(1) {
                println!("command: {}", command.as_str());
                match command.as_str() {
                    "$ ls" => (), // results handled elsewhere
                    _ => { // only other command is cd
                        let command_cap = command_re.captures(line).unwrap();
                        if let Some(cd_directory) = command_cap.get(3) {
                            println!("current_dir before: {}", &current_dir.borrow().name);
                            println!("cd: {}", cd_directory.as_str());
                            match cd_directory.as_str() {
                                "/" => {
                                    current_dir = Rc::clone(&file_system);
                                },
                                ".." => {
                                    let current_dir_clone = Rc::clone(&current_dir);
                                    current_dir = Rc::clone(current_dir_clone.borrow().parent.as_ref().unwrap());
                                },
                                d => {
                                    let new_dir = current_dir.borrow().find_dir(d).unwrap(); // yeah we should panic if it doesn't exist
                                    current_dir = Rc::clone(&new_dir);
                                },
                            }
                            println!("current_dir after: {}", &current_dir.borrow().name);
                        }
                    },
                }
            }
            if let Some(directory) = line_cap.get(2) { // these are directories from results of ls so create missing dirs
                let dir_name_cap = directory_re.captures(directory.as_str()).unwrap();
                dbg!(&dir_name_cap);
                if let Some(dir_name) = dir_name_cap.get(1) {
                    println!("create or find dir: {}", dir_name.as_str());
                    let new_dir = current_dir.borrow().find_dir(dir_name.as_str());
                    match new_dir {
                        Some(_) => (), // directory already exists so do nothing
                        None => {
                            println!("not found so create: {}", dir_name.as_str());
                            let new_dir = Rc::new(RefCell::new(Dir {name: String::from(dir_name.as_str()),
                                files: vec![],
                                dirs: vec![],
                                parent: Some(Rc::clone(&current_dir))}));
                            current_dir.borrow_mut().dirs.push(new_dir);
                            dbg!(&current_dir.borrow().dirs.len());
                        }
                    }
                }
            }
            if let Some(file) = line_cap.get(3) {  // files in results of ls so create missing files
                let file_cap = file_re.captures(file.as_str()).unwrap();
                let new_file = Box::new(File {name: file_cap.get(2).unwrap().as_str().to_string(), size: file_cap.get(1).unwrap().as_str().parse::<i64>().unwrap()});
                dbg!(&new_file);
                current_dir.borrow_mut().files.push(new_file);
            }
        }
        // setup complete now do the problem...
        // part1
        let mut all_dirs = vec![];
        file_system.borrow().all_dirs(&mut all_dirs);
        let all_dirs_part1: Vec<i64> = all_dirs.iter().filter_map(|dir| {
            let size = dir.borrow().get_size();
            match size {
                1..=100000 => Some(size),
                _ => None
            }
        } ).collect();
        dbg!(all_dirs_part1.iter().sum::<i64>());

        // part2
        let file_system_size = 70000000;
        let update_size = 30000000;
        let available_space = file_system_size - file_system.borrow().get_size();
        let space_needed = update_size - available_space;
        dbg!(space_needed);
        dbg!(available_space);
        let all_dirs_part2: Vec<i64> = all_dirs.iter().filter_map(|dir| {
            let size = dir.borrow().get_size();
            if size >= space_needed {
                Some(size)
            }
            else {
                None
            }
        }).collect();
        dbg!(&all_dirs_part2);
        dbg!(all_dirs_part2.iter().min());
    }
}
