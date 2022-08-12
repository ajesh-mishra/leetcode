struct Solution {}

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        use std::char::from_digit;
        let radix = 7;
        let mut result = vec![];
        let mut is_negative = false;

        if num.is_negative() {
            is_negative = true;
            num = -1 * num;
        } else if num == 0 {
            return String::from("0");
        };

        while num > 0 {
            let m = (num % radix) as u32;
            num = num / radix;
            result.push(from_digit(m, radix as u32).unwrap());
        }

        if is_negative {
            result.push('-');
        }

        result.into_iter().rev().collect()
    }
}

fn main() {
    println!("{}", Solution::convert_to_base7(100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_positive() {
        assert_eq!(Solution::convert_to_base7(100), String::from("202"));
    }

    #[test]
    fn ut_negative() {
        assert_eq!(Solution::convert_to_base7(-7), String::from("-10"));
    }

    #[test]
    fn ut_zero() {
        assert_eq!(Solution::convert_to_base7(0), String::from("0"));
    }
}
