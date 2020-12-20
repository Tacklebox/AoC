#![allow(dead_code)]
use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

mod solution;

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let mut lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());
    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("{}", solution::part1(&mut lines));
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        let corrected_lines = lines.map(|line| match &line[..] {
            "8: 42" => String::from("8: 42 | 42 8"),
            "11: 42 31" => String::from("11: 42 31 | 42 11 31"),
            _ => line
        });
        println!("{}", solution::part2(corrected_lines));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}
