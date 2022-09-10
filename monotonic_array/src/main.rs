use std::cmp::Ordering;

pub struct Order {
    pub is_increasing: bool,
    pub is_decreasing: bool,
}

impl Order {
    pub fn new() -> Self {
        Order {
            is_increasing: true,
            is_decreasing: true,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut count = 0;
        let mut order = Order::new();

        while count + 1 < len {
            match nums[count].cmp(&nums[count + 1]) {
                Ordering::Equal => {}
                Ordering::Greater => order.is_increasing = false,
                Ordering::Less => order.is_decreasing = false,
            }
            if !order.is_increasing && !order.is_decreasing {
                return false;
            }
            count += 1;
        }

        order.is_increasing || order.is_decreasing
    }
}

fn main() {
    let nums = vec![1, 2, 2, 3];
    println!("{}", Solution::is_monotonic(nums));
    let nums = vec![1, 3, 2];
    println!("{}", Solution::is_monotonic(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let nums = vec![1, 2, 2, 3];
        assert!(Solution::is_monotonic(nums));
        let nums = vec![2, 2, 2];
        assert!(Solution::is_monotonic(nums));
    }

    #[test]
    fn ut_false() {
        let nums = vec![1, 3, 2];
        assert!(!Solution::is_monotonic(nums));
    }
}
