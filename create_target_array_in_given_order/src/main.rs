struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for (&i, &num) in index.iter().zip(nums.iter()) {
            result.insert(i as usize, num);
        }
        result
    }
}

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let index = vec![0, 1, 2, 2, 1];
    println!("{:?}", Solution::create_target_array(nums, index));
}
