use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    #[inline]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut counter: HashSet<i32> = HashSet::new();

        for i in nums {
            if counter.contains(&i) {
                return true;
            } else {
                counter.insert(i);
            }
        }

        false
    }

    #[inline]
    pub fn contains_duplicate_2(nums: Vec<i32>) -> bool {
        let mut counter: Vec<i32> = Vec::new();

        for i in nums {
            if counter.contains(&i) {
                return true;
            } else {
                counter.push(i);
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let numbers = vec![1,2,3,1];
        assert!(Solution::contains_duplicate(numbers));
    }

    #[test]
    fn tc_2() {
        let numbers = vec![1,2,3];
        assert!(!Solution::contains_duplicate(numbers));
    }

    #[test]
    fn tc_3() {
        let numbers = vec![1,1,1,3,3,4,3,2,4,2];
        assert!(Solution::contains_duplicate(numbers));
    }
}