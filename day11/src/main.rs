#[derive(Debug)]
struct ThrownItem {
    item: i64,
    destination: usize
}

#[derive(Debug)]
struct Monkey{
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    divisor: i64,
    true_dest: usize,
    false_dest: usize,
    num_inspected_items: i64,
}

impl Monkey {
    fn new(items: Vec<i64>, operation: fn(i64) -> i64, divisor: i64, true_dest: usize, false_dest: usize) -> Monkey {
        Monkey {
            items, operation, divisor, true_dest, false_dest, num_inspected_items: 0
        }
    }

    fn take_turn(&mut self, modulo: i64) -> Vec<ThrownItem> {
        let mut thrown_items = vec![];
        self.items.reverse();
        while self.items.len() > 0 {
            let item = self.items.pop().unwrap();
            self.num_inspected_items += 1;
            let mut worry_level = (self.operation)(item);
            // worry_level = worry_level / 3;
            worry_level = worry_level % modulo;
            if worry_level % self.divisor == 0 {
                thrown_items.push(ThrownItem {item: worry_level, destination: self.true_dest});
            }
            else {
                thrown_items.push(ThrownItem {item: worry_level, destination: self.false_dest});
            }
        }
        thrown_items
    }
}

fn main() {
    // sample input - easier to enter by hand than to parse
    // let mut monkeys = vec![
    //     Monkey::new(vec![79,98], |i| i * 19, 23, 2, 3),
    //     Monkey::new(vec![54,65,75,74], |i| i + 6, 19, 2, 0),
    //     Monkey::new(vec![79,60,97], |i| i * i, 13, 1, 3),
    //     Monkey::new(vec![74], |i| i + 3, 17, 0, 1),
    // ];

    // puzzle input - easier to enter by hand than to parse
    let mut monkeys = vec![
        Monkey::new(vec![75,63], |i| i * 3, 11, 7, 2),
        Monkey::new(vec![65,79,98,77,56,54,83,94], |i| i + 3, 2, 2, 0),
        Monkey::new(vec![66], |i| i + 5, 5, 7, 5),
        Monkey::new(vec![51,89,90], |i| i * 19, 7, 6, 4),
        Monkey::new(vec![75,94,66,90,77,82,61], |i| i + 1, 17, 6, 1),
        Monkey::new(vec![53,76,59,92,95], |i| i + 2, 19, 4, 3),
        Monkey::new(vec![81,61,75,89,70,92], |i| i * i, 3, 0, 1),
        Monkey::new(vec![81,86,62,87], |i| i + 8, 13, 3, 5),
    ];

    // play rounds
    let modulo_vec: Vec<i64> = monkeys.iter().map(|m| m.divisor).collect();
    let modulo = modulo_vec.iter().product();
    // dbg!(modulo);
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let thrown_items = monkeys[i].take_turn(modulo);
            for item in thrown_items {
                monkeys[item.destination].items.push(item.item);
            }
        }
    }
    let mut inspected_item_counts: Vec<i64> = monkeys.iter().map(|m| m.num_inspected_items).collect();
    inspected_item_counts.sort();
    inspected_item_counts.reverse();
    println!("top 2 monkeys are: {} and {}.  Monkey business = {}",
             inspected_item_counts[0], inspected_item_counts[1], inspected_item_counts[0] * inspected_item_counts[1]);
    // dbg!(monkeys);
}
