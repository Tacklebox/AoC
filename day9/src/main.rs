#![allow(dead_code)]
use std::{cmp::Ordering, collections::HashMap, env, fs::File, io::{self, BufRead}, path::Path};

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let program: Vec<i64> = io::BufReader::new(input_file).lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect();

    if env::args().skip(1).next() == Some(String::from("part1")) {
        if let Some(invalid_item) = part1(25, &program) {
            println!("Invalid XMAS number {}", invalid_item);
        }
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        if let Some(weakness) = part2(&program) {
            println!("XMAS encryption weakness {}", weakness);
        }
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn part1(preamble_length: usize, cipher_text: &Vec<i64>) -> Option<i64> {
    let mut sliding_window = HashMap::<i64,i64>::new();
    for i in 0..preamble_length {
        sliding_window.insert(cipher_text[i], 1);
    }
    for (earliest_block, next_block) in cipher_text.iter().zip(cipher_text[preamble_length..].iter()) {
        let mut found_two_sum = false;
        for value in sliding_window.keys() {
            let search_value = *next_block - *value;
            if let Some(count) = sliding_window.get(&search_value) {
                if (search_value == *value && *count >= 2) || *count >= 1 {
                    found_two_sum = true;
                    break;
                }
            }
        }
        if found_two_sum {
            // or_insert is unecessary because it's guaranteed to be in the map
            *sliding_window.entry(*earliest_block).or_insert(1) -= 1;
            *sliding_window.entry(*next_block).or_insert(0) += 1;
        } else {
            return Some(*next_block);
        }
    }
    None
}

fn part2(cipher_text: &Vec<i64>) -> Option<i64> {
    if let Some(invalid_item) = part1(25, cipher_text) {
        let sub_cipher = &cipher_text[..cipher_text.iter().position(|&a| a == invalid_item).unwrap()];
        for i in 0..(sub_cipher.len()-2) {
            for j in (i+2)..sub_cipher.len() {
                let s = &sub_cipher[i..j];
                match s.iter().sum::<i64>().cmp(&invalid_item) {
                    Ordering::Less => continue,
                    Ordering::Equal => return Some(*s.iter().min().unwrap() + *s.iter().max().unwrap()),
                    Ordering::Greater => break,
                }
            }
        }
    }
    None
}
