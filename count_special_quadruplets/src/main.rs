use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();

        for i in 0..len - 3 {
            for j in i + 1..len - 2 {
                for k in j + 1..len - 1 {
                    for l in k + 1..len {
                        let lhs = nums[i] + nums[j] + nums[k];
                        let rhs = nums[l];
                        match lhs.cmp(&rhs) {
                            Ordering::Equal => count += 1,
                            _ => continue,
                        }
                    }
                }
            }
        }

        count
    }
}

fn main() {
    let nums = vec![1, 2, 3, 6];
    println!("{}", Solution::count_quadruplets(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1, 1, 1, 3, 5];
        assert_eq!(Solution::count_quadruplets(nums), 4);
    }

    #[test]
    fn ut_2() {
        let nums = vec![32, 53, 13, 98, 55, 60, 14];
        assert_eq!(Solution::count_quadruplets(nums), 1);
        let nums = vec![28, 8, 49, 85, 37, 90, 20, 8];
        assert_eq!(Solution::count_quadruplets(nums), 1);
        let nums = vec![9, 6, 8, 23, 39, 23];
        assert_eq!(Solution::count_quadruplets(nums), 2);
    }
}
