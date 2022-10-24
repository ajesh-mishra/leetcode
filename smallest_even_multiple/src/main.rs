struct Solution {}

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            0 => n,
            _ => n * 2,
        }
    }
}

fn main() {
    println!("{}", Solution::smallest_even_multiple(6));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_multiply() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn ut_n() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}