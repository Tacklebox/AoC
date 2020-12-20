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
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());
    if env::args().skip(1).next() == Some(String::from("part1")) {
        let cubes = solution::ConwayCubes::from_lines(lines);
        println!("{}", solution::part1(cubes));
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        let hyper_cubes = solution::ConwayHyperCubes::from_lines(lines);
        println!("{}", solution::part2(hyper_cubes));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}
