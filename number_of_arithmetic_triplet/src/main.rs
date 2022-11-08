use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                match (nums[j] - nums[i]).cmp(&diff) {
                    Ordering::Less => continue,
                    Ordering::Greater => break,
                    Ordering::Equal => {}
                }
                for k in j + 1..len {
                    match (nums[k] - nums[j]).cmp(&diff) {
                        Ordering::Less => continue,
                        Ordering::Greater => break,
                        Ordering::Equal => count += 1,
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let nums = vec![0, 1, 4, 6, 7, 10];
    println!("{}", Solution::arithmetic_triplets(nums, 3));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::arithmetic_triplets(nums, 2), 2);
    }
}
