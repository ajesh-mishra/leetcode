struct Solution {}

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len < 3 {
            return false;
        }
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                if nums[i] + nums[i + 1] == nums[j] + nums[j + 1] {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let nums = vec![4, 2, 4];
    println!("{}", Solution::find_subarrays(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let nums = vec![4, 2, 4];
        assert!(Solution::find_subarrays(nums));
        let nums = vec![0, 0, 0];
        assert!(Solution::find_subarrays(nums));
    }

    #[test]
    fn ut_false() {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(!Solution::find_subarrays(nums));
    }
}
