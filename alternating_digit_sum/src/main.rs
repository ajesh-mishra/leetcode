struct Solution {}

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        format!("{}", n).char_indices().fold(0, |acc, (i, digit)| {
            let digit = digit.to_digit(10).unwrap() as i32;
            match i % 2 {
                0 => acc + digit,
                _ => acc - digit,
            }
        })
    }
    pub fn alternate_digit_sum_1(n: i32) -> i32 {
        let mut result = 0;
        for (i, digit) in format!("{}", n).char_indices() {
            let digit = digit.to_digit(10).unwrap() as i32;
            if i % 2 == 0 {
                result += digit;
            } else {
                result -= digit;
            }
        }
        result as _
    }
}

fn main() {
    println!("{}", Solution::alternate_digit_sum(886996));
    println!("{}", Solution::alternate_digit_sum_1(886996));
}
