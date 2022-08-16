use std::cmp::Ordering;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let original_len = candy_type.len() / 2;
        let unique_candy: HashSet<i32> = candy_type.into_iter().collect();
        let unique_len = unique_candy.len();

        match unique_len.cmp(&original_len) {
            Ordering::Greater => original_len as i32,
            _ => unique_len as i32,
        }
    }
}

fn main() {
    let candy_type = vec![1, 1, 2, 2, 3, 3];
    println!("{}", Solution::distribute_candies(candy_type));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_half_original() {
        let candy_type = vec![1, 1, 2, 3];
        assert_eq!(Solution::distribute_candies(candy_type), 2);
    }

    #[test]
    fn ut_unique() {
        let candy_type = vec![6, 6, 6, 6];
        assert_eq!(Solution::distribute_candies(candy_type), 1);

        let candy_type = vec![
            100000, 0, 100000, 0, 100000, 0, 100000, 0, 100000, 0, 100000, 0,
        ];
        assert_eq!(Solution::distribute_candies(candy_type), 2);
    }
}
