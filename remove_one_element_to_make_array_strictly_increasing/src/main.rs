struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut prev = i32::MIN;
        for num in nums {
            println!("count: {count}, prev: {prev}, num: {num}");
            if prev < num {
                prev = num;
                continue;
            }
            count += 1;
            if count > 1 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let nums = vec![1, 2, 10, 5, 7];
    println!("{}", Solution::can_be_increasing(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let nums = vec![1, 2, 10, 5, 7];
        assert!(Solution::can_be_increasing(nums));
    }

    #[test]
    fn ut_false() {
        let nums = vec![2, 3, 1, 2];
        assert!(!Solution::can_be_increasing(nums));
        let nums = vec![1, 1, 1];
        assert!(!Solution::can_be_increasing(nums));
    }
}
