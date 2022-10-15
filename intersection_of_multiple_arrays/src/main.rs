use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut temp: HashSet<i32> = HashSet::new();
        for (i, num) in nums.iter().enumerate() {
            if i == 0 {
                temp = num.iter().map(|&x| x).collect();
                continue;
            }
            let temp1: HashSet<i32> = num.iter().map(|&x| x).collect();
            temp = temp.intersection(&temp1).map(|&x| x).collect();
        }
        let mut result: Vec<i32> = temp.iter().map(|&x| x).collect();
        result.sort_by(|a, b| a.cmp(b));
        result
    }
}

fn main() {
    let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
    println!("{:?}", Solution::intersection(nums));
}
