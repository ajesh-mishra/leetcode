use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut diff = i32::MAX;
        let mut result = 0;
        for num in nums {
            match diff.cmp(&num.abs()) {
                Ordering::Greater => {
                    diff = num.abs();
                    result = num;
                }
                Ordering::Equal if num > result => result = num,
                _ => {}
            };
        }
        result
    }
}

fn main() {
    let nums = vec![-4, -2, 1, 4, 8];
    println!("{}", Solution::find_closest_number(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![-4, -2, 1, 4, 8];
        assert_eq!(Solution::find_closest_number(nums), 1);
        let nums = vec![2, -1, 1];
        assert_eq!(Solution::find_closest_number(nums), 1);
    }
}
