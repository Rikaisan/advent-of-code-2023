use std::{fs::File, io::Read};

/// This function loads the input data for a certain day and returns it as a String
pub fn read_input_from_day(day: u8) -> String {
    let day = format!("{:02}", day);
    let mut buf = String::new();
    File::open(format!("input/day_{day}.txt"))
        .expect(format!("Error opening input file for day {day}").as_str())
        .read_to_string(&mut buf)
        .expect(format!("Error reading input file for day {day}").as_str());
    return buf
}

/// This function creates a numeric representation from two `char` digits, it works by
/// using the ASCII value of the character and subtracting the ASCII value of `0` to it
/// to bring the number relative to `0`
/// 
/// ## Safety
/// Callers of this function are responsible that these preconditions are satisfied:
/// - `d1` and `d2` are in the range `0..9`
/// 
/// Failing that means the returned number can overflow.
///
/// # Examples
///
/// ```
/// unsafe {
///     assert_eq!(num_from_chars('0', '0'), 0);
///     assert_eq!(num_from_chars('0', '1'), 1);
///     assert_eq!(num_from_chars('1', '0'), 10);
///     assert_eq!(num_from_chars('3', '6'), 36);
/// }
/// ```
#[inline]
pub unsafe fn num_from_chars(d1: char, d2: char) -> u8 {
    (d1 as u8 - b'0') * 10 + d2 as u8 - b'0'
}


#[cfg(test)]
mod tests {
    use super::*;

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
