use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        for num in nums {
            if num % 2 == 0 {
                let count = count_map.entry(num).or_insert(0);
                *count += 1;
            }
        }
        if count_map.is_empty() {
            return -1;
        }
        let &max_occurance = &count_map.values().max().unwrap();
        let mut min_num = i32::MAX;
        for (&num, count) in &count_map {
            if count == max_occurance && num < min_num {
                min_num = num;
            }
        }
        min_num
    }
}

fn main() {
    let nums = vec![0, 1, 2, 2, 4, 4, 1];
    println!("{}", Solution::most_frequent_even(nums));

    let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];
    println!("{}", Solution::most_frequent_even(nums));
}
