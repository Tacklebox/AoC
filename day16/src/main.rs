#![allow(dead_code)]
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

mod solution;

enum ParseState {
    Rules,
    MyTicket,
    NearbyTickets,
}

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines().map(|l| l.unwrap());
    let mut fields: HashMap<String, ((i32, i32), (i32, i32))> = HashMap::new();
    let mut my_ticket: Vec<i32> = Vec::new();
    let mut tickets: Vec<Vec<i32>> = Vec::new();
    let mut state = ParseState::Rules;
    for line in lines {
        match state {
            ParseState::Rules => {
                if line == "" {
                    state = ParseState::MyTicket;
                    continue;
                }
                let parts: Vec<&str> = line.split(": ").collect();
                let rules: Vec<&str> = parts[1].split(" or ").collect();
                let range1: Vec<i32> = rules[0]
                    .split("-")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                let range2: Vec<i32> = rules[1]
                    .split("-")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                fields.insert(
                    String::from(parts[0]),
                    ((range1[0], range1[1]), (range2[0], range2[1])),
                );
            }
            ParseState::MyTicket => {
                if line == "your ticket:" {
                    continue;
                }
                if line == "" {
                    state = ParseState::NearbyTickets;
                    continue;
                }
                my_ticket.append(
                    &mut line
                        .split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }
            ParseState::NearbyTickets => {
                if line == "nearby tickets:" {
                    continue;
                }
                tickets.push(
                    line.split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                )
            }
        }
    }
    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("{}", solution::part1(&fields, &tickets));
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        println!("{}", solution::part2(&fields, &my_ticket, &tickets));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}
