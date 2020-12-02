use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    collections::HashSet,
    env
};

fn main() -> Result<(), io::Error> {
    let target = 2020;
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();
    let entries: Vec<i32> = lines.map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    if env::args().skip(1).next() == Some(String::from("part1")) {
        if let Some((entry1, entry2)) = part1(target, &entries) {
            print!("Two entry result: {}\n", entry1 * entry2);
            std::process::exit(0);
        }
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        if let Some((entry1, entry2, entry3)) = part2(target, &entries) {
            print!("Three entry result: {}\n", entry1 * entry2 * entry3);
            std::process::exit(0);
        }
    } else {
        print!("usage: cargo run (part1 | part2)\n");
        std::process::exit(1);
    }
    Ok(())
}

fn part1(target: i32, entries: &[i32]) -> Option<(i32, i32)> {
    let mut set = HashSet::<i32>::new();
    for entry in entries {
        if set.contains(&(target - entry)) {
            return Some((*entry, target - entry));
        }
        set.insert(*entry);
    }
    None
}

fn part2(target: i32, entries: &[i32]) -> Option<(i32, i32, i32)> {
    for entry in entries {
        if let Some((entry1, entry2)) = part1(target - entry, &entries) {
            return Some((entry1, entry2, *entry));
        }
    }
    None
}
