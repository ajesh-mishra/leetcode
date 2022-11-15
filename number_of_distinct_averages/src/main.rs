use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| a.cmp(b));
        let mut averages = HashSet::new();
        // println!("{:?}", nums);
        let mut i = 0;
        let mut j = nums.len() - 1;

        while i < j {
            // if nums[i] != nums[j] {
            let avg = (nums[i] as f32 + nums[j] as f32) / 2.0;
            // if avg > avg.floor() {
            //     println!("fractional");
            // }
            // println!("{}, {}", nums[i], nums[j]);
            // println!("average: {avg}");
            averages.insert(format!("{avg}"));
            // }
            i += 1;
            j -= 1;
        }
        averages.len() as _
    }
}

fn main() {
    let nums = vec![4, 1, 4, 0, 3, 5];
    println!("{}", Solution::distinct_averages(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        // let nums = vec![1, 100];
        // assert_eq!(Solution::distinct_averages(nums), 1);
        let nums = vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7];
        assert_eq!(Solution::distinct_averages(nums), 5);
    }
}
