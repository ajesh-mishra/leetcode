use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1: HashSet<&i32> = HashSet::from_iter(nums1.iter());
        let nums2: HashSet<&i32> = HashSet::from_iter(nums2.iter());
        let diff1: Vec<i32> = nums1.difference(&nums2).map(|&&x| x).collect();
        let diff2: Vec<i32> = nums2.difference(&nums1).map(|&&x| x).collect();
        vec![diff1, diff2]
    }
}

fn main() {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];
    println!("{:?}", Solution::find_difference(nums1, nums2));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        assert_eq!(
            Solution::find_difference(nums1, nums2),
            vec![vec![3], vec![]]
        );
    }
}
