use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn is_present(nums: &Vec<i32>, n: i32, start: usize, end: usize) -> Option<usize> {
        let mid = (start + end) / 2;
        if mid == start && nums[mid] != n {
            return None;
        }
        match nums[mid].cmp(&n) {
            Ordering::Equal => Some(mid),
            Ordering::Less => Self::is_present(nums, n, mid, end),
            Ordering::Greater => Self::is_present(nums, n, start, mid),
        }
    }
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort_by(|a, b| a.cmp(b));
        let mut start = 0;
        loop {
            if let Some(x) = Self::is_present(&nums, original, start, nums.len()) {
                original *= 2;
                start = x;
            } else {
                return original;
            }
        }
    }
}

fn main() {
    let nums = vec![5, 3, 6, 1, 12];
    println!("{}", Solution::find_final_value(nums, 3));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![2, 7, 9];
        assert_eq!(Solution::find_final_value(nums, 4), 4);

        let nums = vec![2];
        assert_eq!(Solution::find_final_value(nums, 2), 4);
    }
}
