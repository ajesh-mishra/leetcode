struct Solution {}

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        match n % 2 {
            0 => n / 2,
            _ => n,
        }
    }
}

fn main() {
    println!("{}", Solution::number_of_cuts(6));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::number_of_cuts(4), 2);
        assert_eq!(Solution::number_of_cuts(3), 3);
        assert_eq!(Solution::number_of_cuts(14), 7);
    }
}
