use std::cmp::{min, Ordering};

struct Solution {}

impl Solution {
    fn calculate(curr: u8, prev: u8) -> i32 {
        let (small, big) = match curr.cmp(&prev) {
            Ordering::Greater => (prev, curr),
            Ordering::Less => (curr, prev),
            Ordering::Equal => return 0,
        };
        min(big - small, small + (25 - big) + 1) as _
    }
    pub fn min_time_to_type(word: String) -> i32 {
        let mut prev = 0_u8;
        let mut count = 0;
        for c in word.chars() {
            let curr = c as u8 - 97;
            count += Self::calculate(curr, prev) + 1;
            // println!("{prev}, {curr} -> {count}");
            prev = curr;
        }
        count
    }
}

fn main() {
    let word = String::from("abc");
    println!("{}", Solution::min_time_to_type(word));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let word = String::from("bza");
        assert_eq!(Solution::min_time_to_type(word), 7);
        let word = String::from("zjpc");
        assert_eq!(Solution::min_time_to_type(word), 34);
    }
}