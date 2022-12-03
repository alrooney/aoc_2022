use std::fs;
use regex::Regex;

fn main() {
    let file_path = "elf_list.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"^\d+$").unwrap();
    let mut elves = Vec::new();
    let mut current_elf = 0;
    for line in contents.lines() {
        if re.is_match(line) {
            current_elf += line.parse::<i32>().unwrap();
        }
        else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    println!("elves = {:?}", elves);
    elves.sort();
    elves.reverse();
    println!("sorted elves = {:?}", elves);
    println!("elf carrying most calories = {}", elves[0]);
    println!("top 3 elves carrying = {}", elves[0] + elves[1] + elves[2])
}
