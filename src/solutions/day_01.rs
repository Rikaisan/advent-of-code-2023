use std::{fs::File, io::Read};

pub fn run_a(data: impl Into<String>) -> i32 {
    data.into()
        .split('\n')
        .filter_map(|s: &str| {
            let numbers: Vec<char> = s.trim()
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
                        7pqrstsixteen";

    #[test]
    fn test_solution_a() {
        assert_eq!(run_a(DATA), 142);
        assert_eq!(run_a(DATA2), 209);
    }
}