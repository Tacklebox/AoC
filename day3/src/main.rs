#![allow(dead_code)]
use std::{fmt::Display, env, fs::File, io::{self, BufRead}, path::Path};
use ansi_term::Style;

#[derive(Clone)]
enum TileType {
    Tree,
    Open,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Tree => write!(f, "#")?,
            TileType::Open => write!(f, ".")?,
        };
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    // Trees are marked as true
    let mut snowfield = vec![vec![TileType::Open]];
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();
    for (row_index, line) in lines.enumerate() {
        if let Ok(line) = line {
            if row_index > 0 {
                snowfield.push(vec![TileType::Open]);
            }
            for (column_index, space) in line.chars().enumerate() {
                let tile = if space == '.' {
                    TileType::Open
                } else if space == '#' {
                    TileType::Tree
                } else {
                    std::process::exit(1);
                };
                if column_index == 0 {
                    snowfield[row_index][column_index] = tile;
                } else {
                    snowfield[row_index].push(tile);
                }
            }
        }
    }

    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("We encounter {} trees on the way to the airport", part1(&snowfield, (3, 1)));
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        println!("We encounter {} trees on the way to the airport", part2(snowfield));
    } else {
        print!("usage: cargo run (part1 | part2)\n");
        std::process::exit(1);
     }
    Ok(())
}

// I'm considering the top left of the map to be 0,0,
fn part1(map: &Vec<Vec<TileType>>, slope: (usize, usize)) -> i32 {
    let (run, rise) = slope;
    let mut trees_encountered = 0;
    let mut column = 0;
    let mut row = 0;
    let width = map[0].len();
    while row < map.len() {
        match map[row][column] {
            TileType::Tree => trees_encountered += 1,
            _ => ()
        };
        row += rise;
        column = (column + run) % width;
    }
    trees_encountered
}

fn part2(map: Vec<Vec<TileType>>) -> i64 {
    let slope1_1 = part1(&map, (1,1)) as i64;
    let slope3_1 = part1(&map, (3,1)) as i64;
    let slope5_1 = part1(&map, (5,1)) as i64;
    let slope7_1 = part1(&map, (7,1)) as i64;
    let slope1_2 = part1(&map, (1,2)) as i64;
    slope1_1 * slope3_1 * slope5_1 * slope7_1 * slope1_2
}

fn print_map(map: Vec<Vec<TileType>>, location: (usize, usize)) {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if row == location.1 && column == location.0 {
                print!("{}", Style::new().underline().paint(format!("{}", map[row][column])));
            } else {
                print!("{}", map[row][column]);
            }
        }
        print!("\n");
    }
}
