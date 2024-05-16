use crate::utils;

pub fn run_a(data: impl Into<String>) -> i32 {
    todo!()
}

pub fn run_b(data: impl Into<String>) -> i32 {
    todo!()
}

pub fn run_all() {
    const DAY: u8 = 0;
    let buf = utils::read_input_from_day(DAY);
    println!("Day {:02}, exercise A answer: {}", DAY, run_a(&buf));
    println!("Day {:02}, exercise B answer: {}", DAY, run_b(&buf));
}