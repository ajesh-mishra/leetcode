struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut running_total = 0;

        for (i, n) in nums.iter().enumerate() {
            if total - running_total - n == running_total {
                return i as i32;
            }
            running_total += n;
        }

        -1
    }
}

fn main() {
    let nums = vec![1, 7, 3, 6, 5, 6];
    println!("{}", Solution::pivot_index(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_positive_case() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(Solution::pivot_index(nums), 3);

        let nums = vec![2, 1, -1];
        assert_eq!(Solution::pivot_index(nums), 0);

        let nums = vec![1, -1, 2];
        assert_eq!(Solution::pivot_index(nums), 2);

        let nums = vec![-1, -1, -1, -1, -1, 0];
        assert_eq!(Solution::pivot_index(nums), 2);

        let nums = vec![-1, -1, -1, -1, 0, 1];
        assert_eq!(Solution::pivot_index(nums), 1);

        let nums = vec![-1, -1, 0, 0, -1, -1];
        assert_eq!(Solution::pivot_index(nums), 2);
    }

    #[test]
    fn ut_negative_case() {
        let nums = vec![6, 5, 7, 1, 10, 2, 2, 1, 3, 3];
        assert_eq!(Solution::pivot_index(nums), -1);

        let nums = vec![1, 2, 3];
        assert_eq!(Solution::pivot_index(nums), -1);
    }
}
