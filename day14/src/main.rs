use std::{collections::HashMap, env, fs::File, io::{self, BufRead}, path::Path};
use regex::Regex;

#[derive(Default)]
struct Mask {
    zeros: i64,
    ones: i64,
    floating: Vec<i32>,
}

impl Mask {
    fn update(&mut self, s: &str) {
        self.zeros = 0b111111111111111111111111111111111111;
        self.ones = 0;
        self.floating.clear();
        for (offset, c) in s.split(" = ").skip(1).next().unwrap().chars().rev().enumerate() {
            match c {
                '0' => self.zeros &= !(1 << offset),
                '1' => self.ones |= 1 << offset,
                'X' => self.floating.push(offset as i32),
                _ => continue,
            }
        }
    }

    fn mask(&self, number: i64) -> i64 {
        let number = number | self.ones;
        let number = number & self.zeros;
        number
    }

    fn mask_v2(&self, number: i64) -> Vec<i64> {
        let base_address = number | self.ones;
        let num_perms = usize::pow(2, self.floating.len() as u32);
        let mut addresses: Vec<i64> = vec![base_address; num_perms];
        for permutation in  0..num_perms {
            for (bit_number, offset) in self.floating.iter().enumerate() {
                match 1 & permutation >> bit_number {
                    1 => addresses[permutation] |= 1 << offset,
                    0 => addresses[permutation] &= !(1 << offset),
                    _ => panic!("this is news to me"),
                }
            }
        }
        addresses
    }
}

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines: Vec<String> = io::BufReader::new(input_file).lines().map(|l| l.unwrap()).collect();
    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("{}", part1(&lines));
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        println!("{}", part2(&lines));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn part1(code: &Vec<String>) -> i64 {
    let assignment_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut ram: Vec<i64> = vec![0; usize::pow(2,16)];
    let mut mask = Mask::default();
    for line in code {
        if &line[..4] == "mask" {
            mask.update(line);
        } else {
            if let Some(cap) = assignment_regex.captures(line) {
                let address: usize = cap.get(1).unwrap().as_str().parse().unwrap();
                let original_value = cap.get(2).unwrap().as_str().parse().unwrap();
                let masked_value = mask.mask(original_value);
                ram[address] = masked_value;
            }
        }
    }
    ram.iter().sum()
}

fn part2(code: &Vec<String>) -> i64 {
    let assignment_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut ram: HashMap<i64, i64> = HashMap::new();
    let mut mask = Mask::default();
    for line in code {
        if &line[..4] == "mask" {
            mask.update(line);
        } else {
            if let Some(cap) = assignment_regex.captures(line) {
                let original_address = cap.get(1).unwrap().as_str().parse().unwrap();
                let value = cap.get(2).unwrap().as_str().parse().unwrap();
                for address in mask.mask_v2(original_address).iter() {
                    ram.insert(*address, value);
                }
            }
        }
    }
    ram.values().sum()
}
