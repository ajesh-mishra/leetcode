pub struct Solution {}

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for j in 0..(len - 1) {
            for i in 0..(len - j - 1) {
                if nums[i] < nums[i + 1] {
                    let temp = nums[i + 1];
                    nums[i + 1] = nums[i];
                    nums[i] = temp;
                }
            }
        }

        let mut temp = nums[0];
        let mut counter = 0;

        for i in &nums {
            if i < &temp {
                temp = *i;
                counter += 1;
            }

            if counter == 2 {
                return temp;
            }
        }

        nums[0]
    }
}

fn main() {
    let nums = vec![2, 2, 3, 1];
    println!("{}", Solution::third_max(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let nums = vec![2, 5, 8, 2, 3, 1];
        assert_eq!(Solution::third_max(nums), 3);
    }

    #[test]
    fn tc_2() {
        let nums = vec![1, 2];
        assert_eq!(Solution::third_max(nums), 2);
    }

    #[test]
    fn tc_3() {
        let nums = vec![2, 2, 2, 5, 5, 5];
        assert_eq!(Solution::third_max(nums), 5);
    }
}
