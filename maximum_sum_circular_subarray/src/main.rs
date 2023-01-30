use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_running = 0;
        let mut max_total = i32::MIN;
        let mut min_running = 0;
        let mut min_total = i32::MAX;
        let mut total = 0;

        for num in nums {
            max_running = max(max_running + num, num);
            max_total = max(max_total, max_running);
            min_running = min(min_running + num, num);
            min_total = min(min_total, min_running);
            total += num;
        }

        if max_total < 0 {
            return max_total;
        }

        max(total - min_total, max_total)
    }
}

fn main() {
    let nums = vec![1, -2, 3, -2];
    println!("{}", Solution::max_subarray_sum_circular(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1, -2, 3, -2];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 3);
        let nums = vec![5, -3, 5];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 10);
        let nums = vec![-3, -2, -3];
        assert_eq!(Solution::max_subarray_sum_circular(nums), -2);
    }
}
