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
    const DAY: u8 = 1;
    let buf = utils::read_input_from_day(DAY);
    println!("Day {:02}, exercise A answer: {}", DAY, run_a(&buf));
    println!("Day {:02}, exercise B answer: {}", DAY, run_b(&buf));
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