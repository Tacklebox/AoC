#![allow(dead_code)]
use anyhow::{Context, Result};
use std::{
    env,
    error::Error,
    fmt::{self, Display},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug)]
struct FloorMapError;
impl Display for FloorMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FloorMapError occurred")
    }
}

impl Error for FloorMapError {}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Floor,
    Empty,
    Occupied,
}

struct FloorMap {
    grid: Vec<Vec<Space>>,
    width: usize,
    height: usize,
}

impl FloorMap {
    fn from_reader(reader: impl BufRead) -> Result<Self> {
        let grid: Vec<Vec<Space>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => Space::Floor,
                        'L' => Space::Empty,
                        '#' => Space::Occupied,
                        _ => std::process::exit(1),
                    })
                    .collect::<Vec<Space>>()
            })
            .collect();
        let width = grid[0].len();
        let height = grid.len();
        for row in grid.iter() {
            if row.len() != width {
                return Err(FloorMapError).context("Not all lines of input were the same length");
            }
        }
        Ok(FloorMap {
            grid,
            height,
            width,
        })
    }

    fn get(&self, x: usize, y: usize) -> Result<&Space> {
        if x >= self.width || y >= self.height {
            return Err(FloorMapError).context(format!(
                "Attempted to get an out of bounds location: ({}, {})",
                x, y
            ));
        }
        Ok(&self.grid[y][x])
    }

    fn set(&mut self, x: usize, y: usize, val: Space) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(FloorMapError).context(format!(
                "Attempted to get an out of bounds location: ({}, {})",
                x, y
            ));
        }
        self.grid[y][x] = val;
        Ok(())
    }

    fn neighbours(&self, x: usize, y: usize) -> Result<Vec<Space>> {
        if x >= self.width || y >= self.height {
            return Err(FloorMapError).context(format!(
                "Attempted to get an out of bounds location: ({}, {})",
                x, y
            ));
        }
        let mut valid_neigbours: Vec<Space> = Vec::new();
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                if x_offset != 0 || y_offset != 0 {
                    match self.get(
                        (x as i32 + x_offset) as usize,
                        (y as i32 + y_offset) as usize,
                    ) {
                        Ok(space @ Space::Occupied) | Ok(space @ Space::Empty) => {
                            valid_neigbours.push(*space)
                        }
                        _ => (),
                    }
                }
            }
        }
        Ok(valid_neigbours)
    }

    fn neighbours_line_of_sight(&self, x: usize, y: usize) -> Result<Vec<Space>> {
        if x >= self.width || y >= self.height {
            return Err(FloorMapError).context(format!(
                "Attempted to get an out of bounds location: ({}, {})",
                x, y
            ));
        }

        let mut valid_neigbours: Vec<Space> = Vec::new();
        for (x_offset, y_offset) in [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ]
        .iter()
        {
            let mut new_x = x as i32;
            let mut new_y = y as i32;
            loop {
                new_x += x_offset;
                new_y += y_offset;
                match self.get(new_x as usize, new_y as usize) {
                    Ok(space @ Space::Occupied) | Ok(space @ Space::Empty) => {
                        valid_neigbours.push(*space);
                        break;
                    }
                    Err(_) => break,
                    _ => continue,
                }
            }
        }
        Ok(valid_neigbours)
    }

    fn print_map(&self) -> () {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.grid[y][x] {
                    Space::Occupied => print!("#"),
                    Space::Empty => print!("L"),
                    Space::Floor => print!("."),
                }
            }
            println!("");
        }
    }

    fn seats_with_indices(&self) -> Vec<(Space, usize, usize)> {
        let mut seats: Vec<(Space, usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.grid[y][x] {
                    space @ Space::Occupied | space @ Space::Empty => {
                        seats.push((space, x, y));
                    }
                    _ => (),
                }
            }
        }
        seats
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let mut floor_map = FloorMap::from_reader(io::BufReader::new(input_file))?;

    if env::args().skip(1).next() == Some(String::from("part1")) {
        let stable_occupied = part1(&mut floor_map);
        println!("Seats occupied in stable state {}", stable_occupied);
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        let stable_occupied = part2(&mut floor_map);
        println!("Seats occupied in stable state {}", stable_occupied);
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}

fn part1(floor_map: &mut FloorMap) -> i32 {
    let mut updates: Vec<(usize, usize)> = Vec::new();
    loop {
        for (seat, x, y) in floor_map.seats_with_indices().iter() {
            match seat {
                Space::Empty => {
                    if !floor_map
                        .neighbours(*x, *y)
                        .unwrap()
                        .contains(&Space::Occupied)
                    {
                        updates.push((*x, *y));
                    }
                }
                Space::Occupied => {
                    if floor_map
                        .neighbours(*x, *y)
                        .unwrap()
                        .iter()
                        .filter(|&&s| s == Space::Occupied)
                        .count()
                        >= 4
                    {
                        updates.push((*x, *y));
                    }
                }
                _ => unreachable!(),
            }
        }
        if updates.len() == 0 {
            break;
        }
        for (x, y) in updates.drain(..) {
            if let Ok(Space::Empty) = floor_map.get(x, y) {
                floor_map.set(x, y, Space::Occupied).unwrap();
            } else {
                floor_map.set(x, y, Space::Empty).unwrap();
            }
        }
    }
    floor_map
        .seats_with_indices()
        .iter()
        .filter(|(s, _, _)| *s == Space::Occupied)
        .count() as i32
}

fn part2(floor_map: &mut FloorMap) -> i32 {
    let mut updates: Vec<(usize, usize)> = Vec::new();
    loop {
        for (seat, x, y) in floor_map.seats_with_indices().iter() {
            match seat {
                Space::Empty => {
                    if !floor_map
                        .neighbours_line_of_sight(*x, *y)
                        .unwrap()
                        .contains(&Space::Occupied)
                    {
                        updates.push((*x, *y));
                    }
                }
                Space::Occupied => {
                    if floor_map
                        .neighbours_line_of_sight(*x, *y)
                        .unwrap()
                        .iter()
                        .filter(|&&s| s == Space::Occupied)
                        .count()
                        >= 5
                    {
                        updates.push((*x, *y));
                    }
                }
                _ => unreachable!(),
            }
        }
        if updates.len() == 0 {
            break;
        }
        for (x, y) in updates.drain(..) {
            if let Ok(Space::Empty) = floor_map.get(x, y) {
                floor_map.set(x, y, Space::Occupied).unwrap();
            } else {
                floor_map.set(x, y, Space::Empty).unwrap();
            }
        }
    }
    floor_map
        .seats_with_indices()
        .iter()
        .filter(|(s, _, _)| *s == Space::Occupied)
        .count() as i32
}
