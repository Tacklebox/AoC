#![allow(dead_code)]
use std::{fs::File, io::{self, BufRead}, path::Path};
use std::collections::HashMap;


fn main() -> Result<(), io::Error> {
    // Hashmap is like a set but lets you get/set a value along with each element.
    let mut unique_questions = HashMap::<char, i32>::new();
    // Accumulator for final answer
    let mut sum_of_groups = 0;
    // Number of non-blank lines in a row
    let mut size_of_group = 0;

    // Read input
    let input_path = Path::new("./input.txt");
    let input_file = File::open(input_path)?;
    let lines = io::BufReader::new(input_file).lines();

    for line in lines {
        if let Ok(line) = line {
            // After each group, check all of the values we have
            // stored in the hashmap, if any are equal to the number
            // of group members, that means everyone in the group
            // answered yes to that question, so add it to the total
            if line == "" {
                for (_, n) in unique_questions.iter() {
                    if *n == size_of_group {
                        sum_of_groups += 1;
                    }
                }

                // Reset group size and group responses
                size_of_group = 0;
                unique_questions.clear();
            } else {
                // This is a new person in the group
                size_of_group += 1;
                for ch in line.chars() {
                    // Add each of their responses to the map.
                    *unique_questions.entry(ch).or_insert(0) += 1;
                }
            }
        }
    }

    println!("There are {} unique questions", sum_of_groups);
    Ok(())
}

// part1
//
// let mut unique_questions = HashSet::<char>::new();
// let mut sum_of_groups = 0;
// for line in lines {
//     if let Ok(line) = line {
//         if line == "" {
//             sum_of_groups += unique_questions.len();
//             unique_questions.clear();
//         } else {
//             for ch in line.chars() {
//                 unique_questions.insert(ch);
//             }
//         }
//     }
// }

// println!("There are {} unique questions", sum_of_groups);
