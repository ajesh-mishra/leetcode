struct Solution {}

impl Solution {
    pub fn count_k_difference_1(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for (a, b) in nums.iter().zip(nums.iter().skip(1)) {

        }
        0
    }
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let mut count = 0;
        for i in 0..len - 1 {
            for j in i + 1..len {
                if (nums[i] - nums[j]).abs() == k {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let nums = vec![1, 2, 2, 1];
    println!("{}", Solution::count_k_difference(nums, 1));
    let nums = vec![1, 2, 2, 1];
    println!("{}", Solution::count_k_difference_1(nums, 1));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1, 3];
        assert_eq!(Solution::count_k_difference(nums, 3), 0);
    }
}