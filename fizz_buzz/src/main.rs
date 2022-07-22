struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut i = 1;
        let mut j = 1;
        let mut result: Vec<String> = Vec::new();

        for num in 1..=n {
            if i == 3 && j == 5 {
                result.push("FizzBuzz".to_string());
                i = 1;
                j = 1;
            } else if i == 3 {
                result.push("Fizz".to_string());
                i = 1;
                j += 1;
            } else if j == 5 {
                result.push("Buzz".to_string());
                i += 1;
                j = 1;
            } else {
                result.push(format!("{num}"));
                i += 1;
                j += 1;
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::fizz_buzz(15));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare(expected: &Vec<String>, result: &Vec<String>) -> bool {
        if expected.len() != result.len() {
            return false;
        }

        for (e, r) in zip(expected, result) {
            if e != r {
                return false;
            }
        }

        true
    }

    #[test]
    fn tc_1() {
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "Fizz".to_string(),
            "4".to_string(),
            "Buzz".to_string(),
            "Fizz".to_string(),
            "7".to_string(),
            "8".to_string(),
            "Fizz".to_string(),
            "Buzz".to_string(),
            "11".to_string(),
            "Fizz".to_string(),
            "13".to_string(),
            "14".to_string(),
            "FizzBuzz".to_string(),
        ];
        assert!(compare(&expected, &Solution::fizz_buzz(15)));
    }
}
