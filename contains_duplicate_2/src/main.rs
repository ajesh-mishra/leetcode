struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut i = 0;

        while i + 1 < nums.len() {
            let mut j = 1;

            while j <= k as usize && i + j < nums.len() {
                if nums[i] == nums[i + j] {
                    return true;
                }
                j += 1;
            }
            i += 1;
        }

        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    println!("{}", Solution::contains_nearby_duplicate(nums, k));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn tc_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn tc_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn tc_4() {
        let nums = vec![99, 99];
        let k = 2;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }
}
