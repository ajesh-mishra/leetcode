use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        fn inner(start: i32, end: i32, n: i32) -> i32 {
            let mid = (start + end) / 2;
            if mid == start {
                return -1;
            }
            let sum1 = (mid * (mid + 1)) / 2;
            let sum2 = n - sum1 + mid;
            match sum1.cmp(&sum2) {
                Ordering::Equal => mid,
                Ordering::Greater => inner(start, (start + end) / 2, n),
                Ordering::Less => inner((start + end) / 2, end, n),
            }
        }
        match n {
            1 => 1,
            _ => inner(0, n, (n * (n + 1)) / 2)
        }
    }
}

fn main() {
    println!("{}", Solution::pivot_integer(8));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::pivot_integer(49), 35);
    }
}