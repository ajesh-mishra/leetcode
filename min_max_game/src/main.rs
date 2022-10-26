struct Solution {}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut new_nums = vec![];
        for (i, (&a, &b)) in nums
            .iter()
            .step_by(2)
            .zip(nums.iter().skip(1).step_by(2))
            .enumerate()
        {
            match i % 2 {
                0 => new_nums.push(a.min(b)),
                _ => new_nums.push(a.max(b)),
            }
        }
        Self::min_max_game(new_nums)
    }
}

fn main() {
    let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
    println!("{}", Solution::min_max_game(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];
        assert_eq!(Solution::min_max_game(nums), 1);
        let nums = vec![3];
        assert_eq!(Solution::min_max_game(nums), 3);
        let nums = vec![70, 38, 21, 22];
        assert_eq!(Solution::min_max_game(nums), 22);
    }
}
