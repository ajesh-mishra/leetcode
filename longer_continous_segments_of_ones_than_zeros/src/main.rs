use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let get_longest = |ch: char| {
            s.split(ch)
                .fold(usize::MIN, |longest, chunk| max(longest, chunk.len()))
        };
        get_longest('0') > get_longest('1')
    }
}

fn main() {
    let s = String::from("1101");
    println!("{}", Solution::check_zero_ones(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let s = String::from("1101");
        assert!(Solution::check_zero_ones(s));
    }

    #[test]
    fn ut_false() {
        let s = String::from("111000");
        assert!(!Solution::check_zero_ones(s));
        let s = String::from("110100010");
        assert!(!Solution::check_zero_ones(s));
    }
}
