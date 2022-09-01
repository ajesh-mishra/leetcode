struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut index = 0;
        let mut max_sum = f64::MIN;

        while index + k <= nums.len() as i32 {
            let mut sum: f64 = 0.0;

            for i in index..index + k {
                sum += nums[i as usize] as f64;
            }

            if sum > max_sum {
                max_sum = sum;
            }

            index += 1;
        }

        max_sum / k as f64
    }
}

fn main() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    println!("{}", Solution::find_max_average(nums, 4));
}
