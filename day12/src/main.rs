#![allow(dead_code)]
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

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
    x_coordinate: i32,
    y_coordinate: i32,
    waypoint_x_coordinate: i32,
    waypoint_y_coordinate: i32,
}

impl Ferry {
    fn new() -> Self {
        Ferry { history: Vec::new(), x_coordinate: 0, y_coordinate: 0, waypoint_x_coordinate: 10, waypoint_y_coordinate: 1 }
    }
    fn take_action(&mut self, a: NavigationAction) {
        match a {
            NavigationAction::Forward(m) => {
                self.x_coordinate += m * self.waypoint_x_coordinate;
                self.y_coordinate += m * self.waypoint_y_coordinate;
            }
            NavigationAction::Move(d, m) => {
                match d {
                    Direction::North => self.waypoint_y_coordinate += m,
                    Direction::South => self.waypoint_y_coordinate -= m,
                    Direction::West => self.waypoint_x_coordinate -= m,
                    Direction::East => self.waypoint_x_coordinate += m,
                }
            }
            NavigationAction::Rotate(m) => {
                match m % 360 {
                    180 | -180 => {
                        self.waypoint_x_coordinate *= -1;
                        self.waypoint_y_coordinate *= -1;
                    }
                    90 | -270 => {
                        let temp = -self.waypoint_x_coordinate;
                        self.waypoint_x_coordinate = self.waypoint_y_coordinate;
                        self.waypoint_y_coordinate = temp;
                    }
                    -90 | 270 => {
                        let temp = -self.waypoint_y_coordinate;
                        self.waypoint_y_coordinate = self.waypoint_x_coordinate;
                        self.waypoint_x_coordinate = temp;
                    }
                    _ => panic!("Non cardinal directions not allowed")
                }
            }
        }
    }
    fn displacement(&self) -> i32 {
        self.x_coordinate.abs() + self.y_coordinate.abs()
    }
    fn print_status(&self) {
        println!("Current position: ({}, {}), Current waypoint: ({}, {})", self.x_coordinate, self.y_coordinate, self.waypoint_x_coordinate, self.waypoint_y_coordinate);
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
    ferry.print_status();
    for action in io::BufReader::new(input_file).lines().map(|l| NavigationAction::from_string(l.unwrap())) {
        ferry.take_action(action);
        ferry.print_status();
    }
    println!("Manhattan distance from initial position {}", ferry.displacement());
    std::process::exit(0);
}
