#![allow(dead_code)]
use std::{env, fs::File, io::{self, BufRead}, path::Path};

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let program: Vec<String> = io::BufReader::new(input_file).lines().map(|l| l.unwrap()).collect();

    if env::args().skip(1).next() == Some(String::from("part1")) {
        let (acc, _) = part1(&program);
        println!("Final accumulator value {}", acc);
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        println!("Final accumulator value {}", part2(&program));
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn part1(program: &Vec<String>) -> (i32, i32) {
    let mut accumulator = 0;
    let mut visited = vec![false; program.len()];
    let mut program_counter = 0;
    while program_counter < program.len() && !visited[program_counter] {
        let (op, arg) = program[program_counter].split_at(3);
        visited[program_counter] = true;
        let signed_arg: i32 = arg[1..].parse().unwrap();
        match op {
            "acc" => {
                accumulator += signed_arg;
                program_counter += 1;
            },
            "nop" => program_counter += 1,
            "jmp" => program_counter = (program_counter as i32 + signed_arg) as usize,
            _ => unreachable!(),
        }
    }
    (accumulator, program_counter as i32)
}

fn part2(program: &Vec<String>) -> i32 {
    let flip_map: Vec<bool> = program.iter().map(|instruction| &instruction[..3] != "acc").collect();
    for (index, flip) in flip_map.iter().enumerate() {
        if *flip {
            let mut new_program = program.clone();
            new_program[index] = if &program[index][..3] == "nop" {
                format!("jmp{}", &program[index][3..])
            } else {
                format!("nop{}", &program[index][3..])
            };
            let (acc, pc) = part1(&new_program);
            if pc == program.len() as i32 {
                return acc;
            }
        }
    }
    unreachable!("No valid program found");
}
