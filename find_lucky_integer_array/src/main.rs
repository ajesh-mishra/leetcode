use std::{collections::HashMap, cmp::max};

struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        for n in arr {
            let count = count_map.entry(n).or_insert(0);
            *count += 1;
        }
        let mut largest_lucky = -1;
        for (n, i) in count_map {
            if n == i {
                largest_lucky = max(largest_lucky, i);
            }
        }
        largest_lucky
    }
}

fn main() {
    let arr = vec![2, 2, 3, 4];
    println!("{}", Solution::find_lucky(arr));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(Solution::find_lucky(arr), 3);
    }

    #[test]
    fn ut_false() {
        let arr = vec![2, 2, 2, 3, 3];
        assert_eq!(Solution::find_lucky(arr), -1);
    }
}
