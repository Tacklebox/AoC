#![allow(dead_code)]
use std::{fs::File, io::{self, BufRead}, path::Path};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

fn validate_byr(val: &str) -> bool {
    if let Ok(year) = val.parse::<i32>() {
        year >= 1920 && year <= 2002
    } else {
        false
    }
}
fn validate_iyr(val: &str) -> bool {
    if let Ok(year) = val.parse::<i32>() {
        year >= 2010 && year <= 2020
    } else {
        false
    }
}
fn validate_eyr(val: &str) -> bool {
    if let Ok(year) = val.parse::<i32>() {
        year >= 2020 && year <= 2030
    } else {
        false
    }
}
fn validate_hgt(val: &str) -> bool {
    lazy_static! {
        static ref CM_VALIDATOR: Regex = Regex::new(r"^\d{3}cm$").unwrap();
        static ref IN_VALIDATOR: Regex = Regex::new(r"^\d{2}in$").unwrap();
    }
    if CM_VALIDATOR.is_match(val) {
        let height_in_cm = val[0..3].parse::<i32>().unwrap();
        return height_in_cm <= 193 && height_in_cm >= 150;
    }
    if IN_VALIDATOR.is_match(val) {
        let height_in_in = val[0..2].parse::<i32>().unwrap();
        return height_in_in <= 76 && height_in_in >= 59;
    }
    false
}
fn validate_hcl(val: &str) -> bool {
    lazy_static! {
        static ref HCL_VALIDATOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HCL_VALIDATOR.is_match(val)
}
fn validate_ecl(val: &str) -> bool {
    match val {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}
fn validate_pid(val: &str) -> bool {
    lazy_static! {
        static ref PID_VALIDATOR: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    PID_VALIDATOR.is_match(val)
}

fn main() -> Result<(), io::Error> {
    let mut required_fields = HashMap::<&str, u8>::new();
    required_fields.insert("byr", 0b00000001);
    required_fields.insert("iyr", 0b00000010);
    required_fields.insert("eyr", 0b00000100);
    required_fields.insert("hgt", 0b00001000);
    required_fields.insert("hcl", 0b00010000);
    required_fields.insert("ecl", 0b00100000);
    required_fields.insert("pid", 0b01000000);

    let mut valid_passports = 0;
    let mut field_map: u8 = 0;

    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                if field_map == 0b01111111 {
                    valid_passports += 1;
                }
                field_map = 0;
            } else {
                line.split_whitespace().for_each(|field| {
                    let mut key_val = field.split(':');
                    let key = key_val.next().unwrap();
                    let val = key_val.next().unwrap();
                    if let Some(key_mask) = required_fields.get(key) {
                        if match key {
                            "byr" => validate_byr(val),
                            "iyr" => validate_iyr(val),
                            "eyr" => validate_eyr(val),
                            "hgt" => validate_hgt(val),
                            "hcl" => validate_hcl(val),
                            "ecl" => validate_ecl(val),
                            "pid" => validate_pid(val),
                            _ => false
                        } {
                            field_map = field_map | key_mask;
                        }
                    }
                });
            }
            if field_map == 0b01111111 {
                valid_passports += 1;
            }
        }
    }

    println!("There are {} valid passports", valid_passports);
    Ok(())
}
