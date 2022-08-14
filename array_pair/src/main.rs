struct Solution {}

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        nums.sort_by(|a, b| a.cmp(b));

        for i in (0..nums.len()).step_by(2) {
            result += nums[i];
        }

        result
    }
}

fn main() {
    let nums = vec![1, 4, 3, 2];
    println!("{}", Solution::array_pair_sum(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![6, 2, 6, 5, 1, 2];
        assert_eq!(Solution::array_pair_sum(nums), 9);
    }
}
