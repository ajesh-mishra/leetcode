struct Solution {}

impl Solution {
    fn check_even(mut num: i32, mut sum: i32) -> bool {
        if num == 0 {
            return match sum % 2 {
                0 => true,
                _ => false,
            };
        }
        sum += num % 10;
        num /= 10;
        Self::check_even(num, sum)
    }
    pub fn count_even(num: i32) -> i32 {
        let mut count = 0;
        for i in 2..num + 1 {
            if Self::check_even(i, 0) {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    println!("{}", Solution::count_even(30));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::count_even(4), 2);
    }
}