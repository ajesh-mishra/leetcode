use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut result_1 = 1;
        nums.sort_by(|a, b| b.cmp(a));

        for i in 0..3 {
            result_1 *= nums[i];
        }

        let mut result_2 = if nums[0] > 0 {
            nums[0]
        } else {
            return result_1;
        };

        for i in (len - 2)..len {
            result_2 *= nums[i];
        }

        max(result_1, result_2)
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    println!("{}", Solution::maximum_product(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_with_positive() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::maximum_product(nums), 24);
    }

    #[test]
    fn ut_with_negative() {
        let nums = vec![-100, -98, -1, 2, 3, 4];
        assert_eq!(Solution::maximum_product(nums), 39200);

        let nums = vec![-1, -2, -3];
        assert_eq!(Solution::maximum_product(nums), -6);
    }
}
