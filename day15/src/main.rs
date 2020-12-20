use std::{env, io, collections::HashMap};

fn main() -> Result<(), io::Error> {
    let input = vec![7,12,1,0,16,2];
    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("{}", memory_game(&input, 2020));
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        println!("{}", memory_game_faster(&input, 30000000));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn memory_game(starting_numbers: &[i32], limit: usize) -> i32 {
    let mut sequence = Vec::from(starting_numbers);
    while sequence.len() < limit {
        let last = sequence.last().unwrap();
        let next = if let Some(previous_occurence) = sequence.iter().rev().skip(1).position(|el| el == last) {
            previous_occurence as i32 + 1
        } else {
            0
        };
        sequence.push(next);
    }
    sequence.pop().unwrap()
}

fn memory_game_faster(starting_numbers: &[i32], limit: i32) -> i32 {
    // HashMap<number_spoken, last_turn_it_was_spoken
    let mut last_seen: HashMap<i32, i32> = HashMap::new();
    let mut turn = starting_numbers.len() as i32;
    let mut last = *starting_numbers.last().unwrap();
    for (turn, number) in starting_numbers.iter().enumerate() {
        last_seen.insert(*number, turn as i32 + 1);
    }

    while turn < limit {
        let next = if let Some(&previous_occurence) = last_seen.get(&last) {
            turn - previous_occurence
        } else {
            0
        };
        if turn != starting_numbers.len() as i32 {
            last_seen.insert(last, turn);
        }
        turn += 1;
        last = next;
    }
    last
}
