use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let (mut result, mut prev) = (i32::MIN, i32::MIN);
        let mut streak = 0;
        for n in nums {
            if n > prev {
                streak += n;
                prev = n;
                continue;
            }
            result = max(result, streak);
            prev = n;
            streak = n;
        }
        max(result, streak)
    }
}

fn main() {
    let nums = vec![10, 20, 30, 5, 10, 50];
    println!("{}", Solution::max_ascending_sum(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_2() {
        let nums = vec![3, 6, 10, 1, 8, 9, 9, 8, 9];
        assert_eq!(Solution::max_ascending_sum(nums), 19);
    }

    #[test]
    fn ut_1() {
        let nums = vec![10, 20, 30, 40, 50];
        assert_eq!(Solution::max_ascending_sum(nums), 150);
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        assert_eq!(Solution::max_ascending_sum(nums), 33);
    }
}
