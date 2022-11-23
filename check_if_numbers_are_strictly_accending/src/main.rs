use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = i32::MIN;
        for word in s.split(" ") {
            if let Ok(x) = word.parse::<i32>() {
                match prev.cmp(&x) {
                    Ordering::Less => prev = x,
                    _ => return false,
                }
            }
        }
        true
    }
}

fn main() {
    let s = String::from("1 box has 3 blue 4 red 6 green and 12 yellow marbles");
    println!("{}", Solution::are_numbers_ascending(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let s = String::from("1 box has 3 blue 4 red 6 green and 12 yellow marbles");
        assert!(Solution::are_numbers_ascending(s));
    }

    #[test]
    fn ut_false() {
        let s = String::from("sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s");
        assert!(!Solution::are_numbers_ascending(s));
        let s = String::from("hello world 5 x 5");
        assert!(!Solution::are_numbers_ascending(s));
    }
}
