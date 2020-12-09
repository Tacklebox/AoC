use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    collections::HashSet,
    env
};

struct Seat {
    row: i32,
    column: i32,
}

impl Seat {
    fn from_binary_space_partition(bsp: &str) -> Self {
        let mut rows: &[i32] = &(0..128).collect::<Vec<i32>>();
        let mut columns: &[i32] = &(0..8).collect::<Vec<i32>>();
        for ch in bsp.chars() {
            if ch == 'F' {
                rows = &rows[..(rows.len()/2)];
            } else if ch == 'B' {
                rows = &rows[(rows.len()/2)..];
            } else if ch == 'L' {
                columns = &columns[..(columns.len()/2)];
            } else if ch == 'R' {
                columns = &columns[(columns.len()/2)..];
            } else {
                unreachable!();
            }
        }
        Seat { row: rows[0], column: columns[0] }
    }
    fn get_id(&self) -> i32 {
        (self.row * 8) + self.column
    }
}

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();
    let mut boarding_pass_ids: Vec<i32> = lines.map(|line| Seat::from_binary_space_partition(&line.unwrap()).get_id()).collect();
    if env::args().skip(1).next() == Some(String::from("part1")) {
        let max_id =  part1(&boarding_pass_ids);
        println!("Maximum seat ID: {}", max_id);
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        boarding_pass_ids.sort_unstable();
        let my_seat_id = part2(&boarding_pass_ids);
        println!("My seat ID: {}", my_seat_id);
        std::process::exit(0);
    } else {
        print!("usage: cargo run (part1 | part2)\n");
        std::process::exit(1);
    }
}

fn part1(boarding_pass_ids: &[i32]) -> i32 {
    *boarding_pass_ids.iter().max().unwrap()
}

fn part2(boarding_pass_ids: &[i32]) -> i32 {
    for i in 1..boarding_pass_ids.len() {
        if boarding_pass_ids[i] - boarding_pass_ids[i-1] == 2 {
            return boarding_pass_ids[i] - 1;
        }
    }
    unreachable!();
}
