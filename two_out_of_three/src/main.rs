use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    fn map_me(map: &mut HashMap<i32, u32>, nums: HashSet<&i32>) {
        for &n in nums {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }
    }
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();

        Self::map_me(&mut map, HashSet::from_iter(nums1.iter()));
        Self::map_me(&mut map, HashSet::from_iter(nums2.iter()));
        Self::map_me(&mut map, HashSet::from_iter(nums3.iter()));

        for (num, count) in map {
            if count > 1 {
                result.push(num);
            }
        }
        
        result
    }
}

fn main() {
    let nums1 = vec![1, 1, 3, 2];
    let nums2 = vec![2, 3];
    let nums3 = vec![3];
    println!("{:?}", Solution::two_out_of_three(nums1, nums2, nums3));
}
