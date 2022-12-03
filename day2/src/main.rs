use std::fs;
use regex::Regex;

fn main() {
    let file_path = "game_moves.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"^(\w)\s*(\w)$").unwrap();
    let mut score = 0;
    for line in contents.lines() {
        for game_move in re.captures_iter(line) {
            println!("opponent game_move = {} my game move = {}", &game_move[1], &game_move[2]);
            score += play_round_part2(&game_move[1], &game_move[2]);
        }
    }
    println!("Score = {}", score);
}

fn play_round_part1(opponent: &str, me: &str) -> i32 {
    // opponent: rock,paper,scissors = A,B,C
    // me: rock,paper,scissors = X,Y,Z
    // scores_my_hand: rock,paper,scissors = 1,2,3
    // scores_outcome: loss,tie,win = 0,3,6
    // scores_round = scores_my_hand + scores_outcome
    match me {
        "X" => {
            match opponent {
                "A" => 1 + 3, // tie
                "B" => 1 + 0, // lose
                "C" => 1 + 6, // win
                _ => oops(opponent)
            }
        },
        "Y" => {
            match opponent {
                "A" => 2 + 6, // win
                "B" => 2 + 3, // tie
                "C" => 2 + 0, // lose
                _ => oops(opponent)
            }
        },
        "Z" => {
            match opponent {
                "A" => 3 + 0, // lose
                "B" => 3 + 6, // win
                "C" => 3 + 3, // tie
                _ => oops(opponent)
            }
        },
        _ => oops(me)
    }
}

fn play_round_part2(opponent: &str, me: &str) -> i32 {
    // opponent: rock,paper,scissors = A,B,C
    // me: lose,draw,win = X,Y,Z
    // scores_my_hand: rock,paper,scissors = 1,2,3
    // scores_outcome: loss,tie,win = 0,3,6
    // scores_round = scores_my_hand + scores_outcome
    match me {
        "X" => {
            match opponent {
                "A" => 3 + 0, // lose
                "B" => 1 + 0, // lose
                "C" => 2 + 0, // lose
                _ => oops(opponent)
            }
        },
        "Y" => {
            match opponent {
                "A" => 1 + 3, // tie
                "B" => 2 + 3, // tie
                "C" => 3 + 3, // tie
                _ => oops(opponent)
            }
        },
        "Z" => {
            match opponent {
                "A" => 2 + 6, // win
                "B" => 3 + 6, // win
                "C" => 1 + 6, // win
                _ => oops(opponent)
            }
        },
        _ => oops(me)
    }
}

fn oops(input: &str) -> i32 {
    println!("Unexpected input! {}", input);
    0
}
