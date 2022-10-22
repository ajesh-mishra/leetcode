struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let nums_len = nums.len();
        for i in 0..nums_len - 1 {
            for j in i + 1..nums_len {
                // println!("{}, {}", i, j);
                if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let nums = vec![3, 1, 2, 2, 2, 1, 3];
    println!("{}", Solution::count_pairs(nums, 2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::count_pairs(nums, 1), 0);
    }
}
