use std::{fs::File, io::Read};
use crate::utils::num_from_chars;

pub fn run_a(data: impl Into<String>) -> u32 {
    data.into()
        .lines()
        .map(|s: &str| {
            unsafe {
                num_from_chars(
                    s.chars().find(|c| c.is_ascii_digit()).unwrap_or('0'),
                    s.chars().rev().find(|c| c.is_ascii_digit()).unwrap_or('0')
                ) as u32
            }
        }).sum()
}

// TODO: Find a way to refactor this, can't run a simple loop per 'number_name -> key' on a hashmap or it won't give the right answer.
pub fn run_b(data: impl Into<String>) -> i32 {
    data.into()
    .lines()
    .filter_map(|s: &str| {
        let chars: Vec<char> = s.trim().chars().collect();
        let mut replaced = String::new();
        let mut current_idx = 0;
        loop {
            let char = chars.get(current_idx);
            match char {
                Some(c) => match c {
                    'o' => match chars.get(current_idx..current_idx + 3) {
                        Some(word) => if word.iter().collect::<String>().as_str() == "one" { replaced.push('1') },
                        None => (),
                    },
                    't' => match chars.get(current_idx..current_idx + 3) {
                        Some(word) => if word.iter().collect::<String>().as_str() == "two" { replaced.push('2') } else {
                            match chars.get(current_idx..current_idx + 5) {
                                Some(word) => if word.iter().collect::<String>().as_str() == "three" { replaced.push('3') },
                                None => (),
                            }
                        },
                        None => (),
                    },
                    'f' => match chars.get(current_idx..current_idx + 4) {
                        Some(word) => match word.iter().collect::<String>().as_str() { 
                            "four" => { replaced.push('4') },
                            "five" => { replaced.push('5') },
                            _ => (),
                        } 
                        None => (),
                    },
                    's' => match chars.get(current_idx..current_idx + 3) {
                        Some(word) => if word.iter().collect::<String>().as_str() == "six" { replaced.push('6') } else {
                            match chars.get(current_idx..current_idx + 5) {
                                Some(word) => if word.iter().collect::<String>().as_str() == "seven" { replaced.push('7') },
                                None => (),
                            }
                        },
                        None => (),
                    },
                    'e' => match chars.get(current_idx..current_idx + 5) {
                        Some(word) => if word.iter().collect::<String>().as_str() == "eight" { replaced.push('8') },
                        None => (),
                    },
                    'n' => match chars.get(current_idx..current_idx + 4) {
                        Some(word) => if word.iter().collect::<String>().as_str() == "nine" { replaced.push('9') },
                        None => (),
                    },
                    _ => (),
                },
                None => break,
            }
            replaced.push(*char.unwrap());
            current_idx += 1;
        }

        let numbers: Vec<char> = replaced
                                .chars()
                                .filter(|c: &char| c.is_numeric())
                                .collect();
        if !numbers.is_empty() {
            let mut num = String::new();
            num.push(*numbers.first().unwrap());
            num.push(*numbers.last().unwrap());
            return Some(num.parse::<i32>().unwrap());
        }
        return None;
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


    #[test]
    fn test_num_from_chars() {
        unsafe {
            assert_eq!(num_from_chars('0', '0'), 0);
            assert_eq!(num_from_chars('0', '1'), 1);
            assert_eq!(num_from_chars('1', '0'), 10);
            assert_eq!(num_from_chars('8', '0'), 80);
            assert_eq!(num_from_chars('9', '5'), 95);
            assert_eq!(num_from_chars('3', '6'), 36);
        }
    }
}