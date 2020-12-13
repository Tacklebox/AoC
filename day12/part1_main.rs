#![allow(dead_code)]
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
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NavigationAction {
    Rotate(i32),
    Move(Direction, i32),
    Forward(i32)
}

struct Ferry {
    history: Vec<NavigationAction>,
    heading: Direction,
    x_coordinate: i32,
    y_coordinate: i32,
}

impl Ferry {
    fn new() -> Self {
        Ferry { history: Vec::new(), heading: Direction::East, x_coordinate: 0, y_coordinate: 0 }
    }
    fn take_action(&mut self, a: NavigationAction) {
        match a {
            NavigationAction::Forward(m) => self.take_action(NavigationAction::Move(self.heading, m)),
            NavigationAction::Move(d, m) => {
                match d {
                    Direction::North => self.y_coordinate += m,
                    Direction::South => self.y_coordinate -= m,
                    Direction::West => self.x_coordinate -= m,
                    Direction::East => self.x_coordinate += m,
                }
            }
            NavigationAction::Rotate(m) => {
                let direction_order = vec![Direction::North, Direction::East, Direction::South, Direction::West];
                let current_position = direction_order.iter().position(|&d| d == self.heading).unwrap() as i32;
                self.heading = direction_order[((((m % 360) / 90) + 4 + current_position) % 4).abs() as usize];
            }
        }
    }
    fn displacement(&self) -> i32 {
        self.x_coordinate.abs() + self.y_coordinate.abs()
    }
}

impl NavigationAction {
    fn from_string(s: String) -> Self {
        let (action, magnitude) = s.split_at(1);
        match action {
            "N" => NavigationAction::Move(Direction::North, magnitude.parse().unwrap()),
            "S" => NavigationAction::Move(Direction::South, magnitude.parse().unwrap()),
            "E" => NavigationAction::Move(Direction::East, magnitude.parse().unwrap()),
            "W" => NavigationAction::Move(Direction::West, magnitude.parse().unwrap()),
            "L" => NavigationAction::Rotate(-1 * magnitude.parse::<i32>().unwrap()),
            "R" => NavigationAction::Rotate(magnitude.parse().unwrap()),
            "F" => NavigationAction::Forward(magnitude.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;

    let mut ferry = Ferry::new();
    for action in io::BufReader::new(input_file).lines().map(|l| NavigationAction::from_string(l.unwrap())) {
        ferry.take_action(action);
    }
    if env::args().skip(1).next() == Some(String::from("part1")) {
        println!("Manhattan distance from initial position {}", ferry.displacement());
        std::process::exit(0);
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        // let stable_occupied = part2(&mut floor_map);
        // println!("Seats occupied in stable state {}", stable_occupied);
        std::process::exit(0);
    } else {
        println!("usage: cargo run (part1 | part2)");
        std::process::exit(1);
    }
}
