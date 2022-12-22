use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn array_sign_1(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        for num in nums {
            match num.cmp(&0) {
                Ordering::Equal => return 0,
                Ordering::Less => result *= -1,
                Ordering::Greater => {}
            }
        }
        result
    }
}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |acc, &num| acc * num.signum())
    }
}

fn main() {
    let nums = vec![-1, -2, -3, -4, 3, 2, 1];
    println!("{}", Solution::array_sign(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![-1, -2, -3, -4, 3, 2, 1];
        assert_eq!(Solution::array_sign(nums), 1);
        let nums = vec![1, 5, 0, 2, -3];
        assert_eq!(Solution::array_sign(nums), 0);
        let nums = vec![-1, 1, -1, 1, -1];
        assert_eq!(Solution::array_sign(nums), -1);
    }
}
