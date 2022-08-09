use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let sqrt = (num as f64).sqrt() as i32;
        let mut divisors: HashSet<i32> = HashSet::new();
        let mut edge_divisors = 1;

        if num as f32 / sqrt as f32 == sqrt as f32 {
            edge_divisors += sqrt;
        }

        for i in 2..=sqrt + 1 {
            if num % i == 0 {
                divisors.insert(i);
                divisors.insert(num / i);
            }
        }

        if divisors.iter().sum::<i32>() + edge_divisors == num {
            return true;
        }

        false
    }
}

fn main() {
    println!("True -> {}", Solution::check_perfect_number(28));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true_cases() {
        assert!(Solution::check_perfect_number(28));
        assert!(Solution::check_perfect_number(6));
    }

    #[test]
    fn ut_false_cases() {
        assert!(!Solution::check_perfect_number(1));
        assert!(!Solution::check_perfect_number(7));
    }
}
