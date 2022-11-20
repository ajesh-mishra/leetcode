use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in nums {
            let count = map.entry(n).or_insert(0);
            *count += 1;
        }
        // map.values().all(|&c| c % 2 == 0)
        map.iter().all(|(_, c)| c % 2 == 0)
    }
}

fn main() {
    let nums = vec![3, 2, 3, 2, 2, 2];
    println!("{}", Solution::divide_array(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let nums = vec![3, 2, 3, 2, 2, 2];
        assert!(Solution::divide_array(nums));
    }

    #[test]
    fn ut_false() {
        let nums = vec![1, 2, 3, 4];
        assert!(!Solution::divide_array(nums));
    }
}
