use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut positive = BinaryHeap::new();
        let mut negative = BinaryHeap::new();
        for n in nums {
            match n >= 0 {
                true => positive.push(n),
                false => negative.push(n * -1),
            }
        }
        let mut p = positive.pop();
        let mut n = negative.pop();
        while !p.is_none() && !n.is_none() {
            let p_num = p.unwrap();
            let n_num = n.unwrap();
            match p_num.cmp(&n_num) {
                Ordering::Equal => return p_num,
                Ordering::Greater => p = positive.pop(),
                Ordering::Less => n = negative.pop(),
            }
        }
        -1
    }
}

fn main() {
    let nums = vec![-1, 10, 6, 7, -7, 1];
    println!("{}", Solution::find_max_k(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_positive() {
        let nums = vec![-1, 2, -3, 3];
        assert_eq!(Solution::find_max_k(nums), 3)
    }

    #[test]
    fn ut_negative() {
        let nums = vec![-10, 8, 6, 7, -2, -3];
        assert_eq!(Solution::find_max_k(nums), -1)
    }
}
