struct Solution {}

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j + 1..len {
                    if nums[j] != nums[k] && nums[i] != nums[k] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let nums = vec![4, 4, 2, 4, 3];
    println!("{}", Solution::unequal_triplets(nums));
}
