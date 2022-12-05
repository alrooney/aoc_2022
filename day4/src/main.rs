use std::fs;
use regex::Regex;

fn main() {
    let file_path = "pair_list.txt";

    let contents = fs::read_to_string(file_path).expect("cannot open file");
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut num_contained_pairs = 0;
    let mut num_overlapping_pairs = 0;
    for line in contents.lines() {
        for pairs in re.captures_iter(line) {
            let (s1,e1,s2,e2) = (&pairs[1].parse::<i32>().unwrap(), &pairs[2].parse::<i32>().unwrap(),
                                 &pairs[3].parse::<i32>().unwrap(), &pairs[4].parse::<i32>().unwrap());
            println!("s1 = {} e1 = {} s2 = {} e2 = {}", s1,e1,s2,e2);
            num_contained_pairs += contains(*s1,*e1,*s2,*e2);
            num_overlapping_pairs += overlap(*s1,*e1,*s2,*e2);
        }
    }
    println!("num_contained_pairs = {}", num_contained_pairs);
    println!("num_overlapping_pairs = {}", num_overlapping_pairs);
}

fn contains(s1 : i32, e1 : i32, s2 : i32, e2 : i32) -> i32 {
    if ((s1 - s2) <= 0 && (e1 - e2) >= 0) || ((s1 - s2) >= 0 && (e1 - e2) <= 0) {
        1
    }
    else {
        0
    }
}

fn overlap(s1 : i32, e1 : i32, s2 : i32, e2 : i32) -> i32 {
    if (s1 <= s2 && e1 >= s2) || (s1 >= s2 && e2 >= s1) {
        1
    }
    else {
        0
    }
}
