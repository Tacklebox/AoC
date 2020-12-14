#![allow(dead_code)]
use std::{thread, sync::mpsc::channel, env, fs::File, io::{self, BufRead}, path::Path};
use ring_algorithm::chinese_remainder_theorem;

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let mut lines = io::BufReader::new(input_file).lines();
    let earliest_departure = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let bus_id_string = lines.next().unwrap().unwrap();
    let bus_ids: Vec<&str> = bus_id_string.split(',').collect();
    if env::args().skip(1).next() == Some(String::from("part1")) {
        let (earliest_bus_id, wait_time) = part1(earliest_departure, &bus_ids);
        println!("{}", earliest_bus_id * wait_time);
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        let timestamp = part2(&bus_ids);
        println!("timestamp {}", timestamp);
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn part1(earliest_departure: i64, bus_ids: &Vec<&str>) -> (i64, i64) {
    let (bus_index, wait_time) = bus_ids.iter().enumerate().filter_map(|(index, id)| if id == &"x" {
        None
    } else {
        let loop_time = id.parse::<i64>().unwrap();
        Some((index, loop_time - (earliest_departure % loop_time)))
    }).min_by_key(|(_, wait)| *wait).unwrap();
    (bus_ids[bus_index].parse().unwrap(), wait_time)
}

// This will run the examples but not the real input.
fn part2slow(bus_ids: &Vec<&str>) -> i64 {
    let indexed_bus_ids: Vec<(usize, i64)> = bus_ids.iter().enumerate().filter_map(|(index, id)| if id == &"x" {
        None
    } else {
        Some((index, id.parse::<i64>().unwrap()))
    }).collect();
    let (tx, rx) = channel();
    for thread_index in 0..12 {
        let tx = tx.clone();
        let indexed_bus_ids = indexed_bus_ids.clone();
        thread::spawn(move|| {
            for multiple in ((5263157894736 + thread_index)..).step_by(12) {
                let timestamp = indexed_bus_ids[0].1  * multiple;
                if indexed_bus_ids[1..].iter().all(|&(index, bus_id)| (timestamp + index as i64) % bus_id == 0) {
                    tx.send(timestamp).unwrap();
                }
            }
        });
    }
    rx.recv().unwrap()
}

// Kinda cheating but whatever
fn part2(bus_ids: &Vec<&str>) -> i64 {
    let indexed_bus_ids: Vec<(i64, i64)> = bus_ids.iter().enumerate().filter_map(|(index, id)| if id == &"x" {
        None
    } else {
        Some((index as i64, id.parse::<i64>().unwrap()))
    }).collect();
    let mut u = Vec::new();
    let mut m = Vec::new();
    for (index, bus_id) in indexed_bus_ids {
        u.push(-index);
        m.push(bus_id);
    }
    chinese_remainder_theorem(&u, &m).unwrap()
}
