use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut result = -1;
        let len = nums.len();
        for i in 0..len - 1 {
            for j in i + 1..len {
                result = max(result, nums[j] - nums[i]);
            }
        }
        if result == 0 {
            return -1;
        }
        result
    }
}

fn main() {
    let nums = vec![7, 1, 5, 4];
    println!("{}", Solution::maximum_difference(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![9, 4, 3, 2];
        assert_eq!(Solution::maximum_difference(nums), -1);
        let nums = vec![1, 5, 2, 10];
        assert_eq!(Solution::maximum_difference(nums), 9);
    }
}
