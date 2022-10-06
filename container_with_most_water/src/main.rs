use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        
        while left < right {
            let distance = (right - left) as i32;
            let length: i32;
            
            if height[left] < height[right] {
                length = height[left];
                left += 1;
            } else {
                length = height[right];
                right -= 1;
            }

            result = max(result, distance * length);
        }

        result
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("{}", Solution::max_area(height));
}
