struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

fn main() {
    println!("{}", Solution::can_win_nim(4));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn tc_2() {
        assert!(Solution::can_win_nim(5));
    }

    #[test]
    fn tc_3() {
        assert!(!Solution::can_win_nim(16));
    }
}