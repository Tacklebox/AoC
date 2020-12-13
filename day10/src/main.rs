#![allow(dead_code)]
use std::{collections::HashMap, env, fs::File, io::{self, BufRead}, path::Path};

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let mut adaptors: Vec<i64> = io::BufReader::new(input_file).lines().map(|l| l.unwrap().parse::<i64>().unwrap()).collect();
    adaptors.sort();
    adaptors.insert(0,0);
    adaptors.push(adaptors.last().unwrap() + 3);

    if env::args().skip(1).next() == Some(String::from("part1")) {
        if let Some(( low, _, high )) = part1(&adaptors) {
            println!("Product of low and high joltage differences {}", low * high);
        }
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        let mut memo: HashMap<i64, i64> = HashMap::new();
        let permutations = part2(&adaptors, &mut memo);
        println!("permutations {}", permutations);
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

// Returns none if not sorted or larger than 3 jolt gap
fn part1(adaptors: &Vec<i64>) -> Option<(i64, i64, i64)> {
    let mut one_jolt_diff = 0;
    let mut two_jolt_diff = 0;
    let mut three_jolt_diff = 0;
    for pair in adaptors.windows(2) {
        if pair[1] <= pair[0] {
            return None;
        }
        match pair[1] - pair[0] {
            1 => one_jolt_diff += 1,
            2 => two_jolt_diff += 1,
            3 => three_jolt_diff += 1,
            _ => return None,
        }
    }
    Some((one_jolt_diff, two_jolt_diff, three_jolt_diff))
}

fn part2(adaptors: &[i64], memo: &mut HashMap<i64,i64>) -> i64 {
    if adaptors.len() <= 2 {
        return 1
    }
    if let Some(permutations) = memo.get(&adaptors[0]) {
        return *permutations;
    }
    let mut permutations = 0;
    for (index, adaptor) in adaptors[1..].iter().take(3).enumerate() {
        if adaptor - adaptors[0] <= 3 {
            permutations += part2(&adaptors[index+1..], memo);
        }
    }
    memo.insert(adaptors[0], permutations);
    permutations
}
