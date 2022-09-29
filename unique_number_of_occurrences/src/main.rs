use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count_map: HashMap<i32, u32> = HashMap::new();
        for n in arr {
            let count = count_map.entry(n).or_insert(0);
            *count += 1;
        }

        count_map.len()
            == count_map
                .values()
                .map(|&x| x)
                .collect::<HashSet<u32>>()
                .len()
    }
}

fn main() {
    let arr = vec![1, 2, 2, 1, 1, 3];
    println!("{}", Solution::unique_occurrences(arr));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(Solution::unique_occurrences(arr));
    }

    #[test]
    fn ut_false() {
        let arr = vec![1, 2];
        assert!(!Solution::unique_occurrences(arr));
    }
}
