use std::fs;
use regex::Regex;

#[derive(Clone, Debug)]
struct Stack {
   crates : Vec<char>
}

fn main() {
    let is_part1 = false;
    let file_path = "move_list.txt";

    let contents = fs::read_to_string(file_path).expect("cannot open file");

    let mut stacks : Vec<Stack> = vec![];
    let mut are_stacks_complete = false;
    for line in contents.lines() {
        if line == "" {
            for i in 0..stacks.len() {
                stacks[i].crates.pop();
                stacks[i].crates.reverse();
            }
            are_stacks_complete = true;
            println!("{:?}", stacks);
            continue;
        }
        if !are_stacks_complete {
            // crates are at 1 + 4n (n = 0...inf)
            let len = line.len();
            let num_stacks = (len - 1)/4 + 1;
            println!("len = {} num_stacks = {}", len, num_stacks);
            while num_stacks > stacks.len() {
                let crates : Vec<char> = vec![];
                stacks.push(Stack {crates});
            }
            let crates : Vec<char> = line.chars().collect();
            for stack in 0..num_stacks {
                let crate_in_stack = crates[1 + 4 * stack];
                if crate_in_stack != ' ' {
                    stacks[stack].crates.push(crate_in_stack);
                }
            }
        }
        else {
            let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
            for instruction in re.captures_iter(line) {
                let num_to_move = instruction[1].parse::<usize>().unwrap();
                let source = instruction[2].parse::<usize>().unwrap() - 1;
                let destination = instruction[3].parse::<usize>().unwrap() - 1;
                if is_part1 {
                    crane_9000_aka_part1(&mut stacks, num_to_move, source, destination);
                }
                else {
                    crane_9001_aka_part2(&mut stacks, num_to_move, source, destination);
                }
            }
        }
    }
    let mut code = String::from("");
    for stack in stacks {
        code.push(stack.crates.last().unwrap().clone());
    }
    println!("{}", code);
}

fn crane_9000_aka_part1(stacks: &mut Vec<Stack>, num_to_move: usize, source: usize, destination: usize) {
    for _i in 0..num_to_move {
        let crate_in_stack = stacks[source].crates.pop().unwrap();
        stacks[destination].crates.push(crate_in_stack);
    }
}

fn crane_9001_aka_part2(stacks: &mut Vec<Stack>, num_to_move: usize, source: usize, destination: usize) {
    let len = stacks[source].crates.len();
    assert!(len >= num_to_move);
    let mut crates_in_stack = stacks[source].crates.split_off(len - num_to_move);
    stacks[destination].crates.append(&mut crates_in_stack);
}
