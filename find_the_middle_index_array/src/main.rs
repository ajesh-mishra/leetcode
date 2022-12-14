struct Solution {}

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let index = 0;
        let right: i32 = nums.iter().sum::<i32>() - *nums.first().unwrap();
        let left = 0;
        fn inner(nums: &Vec<i32>, mut index: usize, mut left: i32, mut right: i32) -> i32 {
            if left == right {
                return index as i32;
            }
            if index == nums.len() - 1 {
                return -1;
            }
            left += nums.get(index).unwrap();
            index += 1;
            right -= nums.get(index).unwrap();
            inner(nums, index, left, right)
        }

        inner(&nums, index, left, right)
    }
}

fn main() {
    let nums = vec![2, 3, -1, 8, 4];
    println!("{}", Solution::find_middle_index(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1,-1,4];
        assert_eq!(Solution::find_middle_index(nums), 2);
        let nums = vec![2,5];
        assert_eq!(Solution::find_middle_index(nums), -1);
        let nums = vec![0,0,0,0];
        assert_eq!(Solution::find_middle_index(nums), 0);
    }
}