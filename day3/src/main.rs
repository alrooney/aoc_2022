use std::fs;

#[derive(Clone, Debug)]
struct Rucksack {
    items: Vec<char>
}

fn main() {
    let file_path = "packing_list.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut group : Vec<Rucksack> = vec![];
    let mut score_part1 = 0;
    let mut score_part2 = 0;
    for line in contents.lines() {
        let items : Vec<char> = line.chars().collect();
        score_part1 += score_rucksack_part1(items.clone());
        group.push(Rucksack {items});
        if group.len() > 2 {
            score_part2 += score_rucksack_part2(group.clone());
            group.clear();
        }
    }
    println!("Score for part1 = {}", score_part1);
    println!("Score for part2 = {}", score_part2);
}

fn score_rucksack_part1(packing_list: Vec<char>) -> usize {
    // split packing_list in half
    // find letter common in both halves
    // score the item/letter that is in both halves
    // a..z = 1..26 and A..Z = 27..52

    let (compartment1, compartment2) = packing_list.split_at(packing_list.len()/2);
    let mut compartment1 = compartment1.to_vec();
    let mut compartment2 = compartment2.to_vec();

    compartment1.sort();
    compartment2.sort();

    // println!("compartment1 = {:?}", compartment1);
    // println!("compartment2 = {:?}", compartment2);

    let mut shared_item = None;

    while shared_item == None {
        // println!("c1.last() = {} c2.last() = {}", compartment1.last().unwrap(), compartment2.last().unwrap());
        // println!("c1 = {:?} c2 = {:?}", compartment1, compartment2);
        if compartment1.last().unwrap() >  compartment2.last().unwrap() {
            compartment1.pop();
            continue;
        }
        if compartment2.last().unwrap() > compartment1.last().unwrap() {
            compartment2.pop();
            continue;
        }
        // otherwise they are equal and we have found our shared item
        shared_item = Some(compartment2[compartment2.len() - 1]);
    }

    // println!("shared_item is {}", shared_item.unwrap());

    let chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    // println!("chars = {:?} chars.len() = {}", chars, chars.len());

    let index = chars.iter().position(|&c| c == shared_item.unwrap()).unwrap();

    // println!("index = {}", index);

    index + 1
}

fn score_rucksack_part2(group: Vec<Rucksack>) -> usize {
    let mut min_len = 0;
    let mut sorted_group : Vec<Rucksack> = vec![];
    for mut rucksack in group {
        rucksack.items.sort();
        if rucksack.items.len() < min_len || min_len == 0 {
            min_len = rucksack.items.len();
        }
        // println!("{:?}", rucksack);
        sorted_group.push(rucksack);
    }
    let mut shared_item : Option<char> = None;
    while shared_item == None && sorted_group[0].items.len() > 0 {
        if sorted_group[0].items.last() == sorted_group[1].items.last() && sorted_group[0].items.last() == sorted_group[2].items.last() {
            shared_item = Some(sorted_group[0].items.last().unwrap().clone());
            continue;
        }
        if sorted_group[0].items.last() > sorted_group[1].items.last() || sorted_group[0].items.last() > sorted_group[2].items.last() {
            sorted_group[0].items.pop();
            continue;
        }
        if sorted_group[1].items.last() > sorted_group[0].items.last() || sorted_group[1].items.last() > sorted_group[2].items.last() {
            sorted_group[1].items.pop();
            continue;
        }
        if sorted_group[2].items.last() > sorted_group[0].items.last() || sorted_group[2].items.last() > sorted_group[1].items.last() {
            sorted_group[2].items.pop();
            continue;
        }
    }
    println!("shared_item_part2 = {}", shared_item.unwrap());
    let chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let index = chars.iter().position(|&c| c == shared_item.unwrap()).unwrap();
    index + 1
}
