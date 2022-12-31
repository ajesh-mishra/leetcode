struct Solution {}

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        result.append(&mut nums);
        result
    }
}

fn main() {
    let nums = vec![1, 2, 1];
    println!("{:?}", Solution::get_concatenation(nums));
}
