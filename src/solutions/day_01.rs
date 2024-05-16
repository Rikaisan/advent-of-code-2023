use std::{fs::File, io::Read};
use crate::utils;

pub fn run_a(data: impl Into<String>) -> u32 {
    data.into()
        .lines()
        .map(|s: &str| {
            unsafe {
                utils::num_from_chars(
                    s.chars().find(|c| c.is_ascii_digit()).unwrap_or('0'),
                    s.chars().rev().find(|c| c.is_ascii_digit()).unwrap_or('0')
                ) as u32
            }
        }).sum()
}

// TODO: Find a way to refactor this, can't run a simple loop per 'number_name -> key' on a hashmap or it won't give the right answer.
pub fn run_b(data: impl Into<String>) -> u32 {
    const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    data.into()
    .lines()
    .map(|s: &str| {
        unsafe {
            utils::num_from_chars(
                s.chars()
                .enumerate()
                .find_map(|(idx, c)| { 
                    c.is_ascii_digit()
                    .then_some(c)
                    .or(NUMBERS.iter()
                        .enumerate()
                        .find(|(_, num)| s[idx..].starts_with(**num))
                        .map(|(n_idx, _)| (n_idx as u8 + 1 as u8 + b'0') as char)
                    )
                }).unwrap_or('0'),
                s.chars()
                .rev()
                .enumerate()
                .find_map(|(idx, c)| { 
                    c.is_ascii_digit()
                    .then_some(c)
                    .or(NUMBERS.iter()
                        .enumerate()
                        .find(|(_, num)| s[s.len() - 1 - idx..].starts_with(**num))
                        .map(|(n_idx, _)| (n_idx as u8 + 1 as u8 + b'0') as char) 
                    )
                }).unwrap_or('0')
            ) as u32
        }
    }).sum()
}

pub fn run_all() {
    let mut buf = String::new();
    File::open("input/day_01.txt")
        .expect("Error opening input file for day 01")
        .read_to_string(&mut buf)
        .expect("Error reading input file for day 01");
    println!("Day 01, exercise A answer: {}", run_a(&buf));
    println!("Day 01, exercise B answer: {}", run_b(&buf));
}

#[cfg(test)]
mod tests {
    use super::*;

    // These include the indentation spaces for some reason even though they are not raw strings, but doesn't matter for testing purposes.
    const DATA: &str = "1abc2
                        pqr3stu8vwx
                        a1b2c3d4e5f
                        treb7uchet";

    const DATA2: &str = "two1nine
                        eightwothree
                        abcone2threexyz
                        xtwone3four
                        4nineeightseven2
                        zoneight234
                        7pqrstsixteen
                        eightwo2twone";

    #[test]
    fn test_solution_a() {
        assert_eq!(run_a(DATA), 142);
        assert_eq!(run_a(DATA2), 231);
    }

    #[test]
    fn test_solution_b() {
        assert_eq!(run_b(DATA), 142);
        assert_eq!(run_b(DATA2), 362);
    }
}