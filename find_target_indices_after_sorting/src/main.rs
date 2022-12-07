struct Solution {}

impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_by(|a, b| a.cmp(b));
        let mut result = vec![];
        let mut found = false;
        for (i, n) in nums.iter().enumerate() {
            match (n, found) {
                (n, _) if n == &target => {
                    result.push(i as i32);
                    found = true;
                }
                (n, true) if n != &target => break,
                (_, _) => {}
            }
        }
        result
    }
}

fn main() {
    let nums = vec![1, 2, 5, 2, 3];
    println!("{:?}", Solution::target_indices(nums, 2));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![1, 2, 5, 2, 3];
        assert_eq!(Solution::target_indices(nums, 3), vec![3]);
        let nums = vec![1, 2, 5, 2, 3];
        assert_eq!(Solution::target_indices(nums, 5), vec![4]);
    }
}
