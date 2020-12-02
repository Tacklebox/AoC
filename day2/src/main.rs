use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    env,
    error::Error,
    fmt,
};

struct PasswordEntry {
    lower_bound: i32,
    upper_bound: i32,
    rule: char,
    password: String,
}

#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error!")
    }
}

impl Error for ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseError
    }
}

impl PasswordEntry {
    fn from_string(string_to_parse: String) -> Result<Self, ParseError> {
        let parts: Vec<&str> = string_to_parse.split(" ").collect();
        let bounds: Vec<&str> = parts[0].split("-").collect();
        let lower_bound = bounds[0].parse::<i32>()?;
        let upper_bound = bounds[1].parse::<i32>()?;
        let rule = parts[1].chars().next().ok_or(ParseError)?;
        Ok(PasswordEntry { lower_bound, upper_bound, rule, password: String::from(parts[2]) })
    }
}

fn main() -> Result<(), io::Error> {
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();
    let mut entries: Vec<PasswordEntry> = vec![];
    for line in lines {
        if let Ok(line) = line {
            if let Ok(entry) = PasswordEntry::from_string(line) {
                entries.push(entry);
            }
        }
    }
    if env::args().skip(1).next() == Some(String::from("part1")) {
        print!("valid passwords: {}\n", part1(entries));
    } else if env::args().skip(1).next() == Some(String::from("part2")) {
        print!("valid passwords: {}\n", part2(entries));
    } else {
        print!("usage: cargo run (part1 | part2)\n");
        std::process::exit(1);
    }
    Ok(())
}

fn part1(entries: Vec<PasswordEntry>) -> i32 {
    let mut valid_passwords = 0;
    for entry in entries.iter() {
        let mut num_rule_chars_found = 0;
        for ch in entry.password.chars() {
            if ch == entry.rule {
                num_rule_chars_found += 1;
            }
        }
        if num_rule_chars_found >= entry.lower_bound && num_rule_chars_found <= entry.upper_bound {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn part2(entries: Vec<PasswordEntry>) -> i32 {
    let mut valid_passwords = 0;
    for entry in entries.iter() {
        let chars: Vec<char> = entry.password.chars().collect();
        let lower_match = chars[(entry.lower_bound - 1) as usize] == entry.rule;
        let upper_match = chars[(entry.upper_bound - 1) as usize] == entry.rule;
        if (lower_match || upper_match) && !(lower_match && upper_match){
            valid_passwords += 1;
        }
    }
    valid_passwords
}
