struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        for (i, n1) in nums1.iter().enumerate() {
            let mut pos: usize = 0;
            if let Some(p) = nums2.iter().position(|&x| x == *n1) {
                pos = p;
            }
            for p in pos..nums2.len() {
                if nums2[p] > *n1 {
                    answer.push(nums2[p]);
                    break;
                }
            }
            if answer.len() != i + 1 {
                answer.push(-1);
            }
        }
        answer
    }
}

fn main() {
    let nums1 = vec![4, 1, 2];
    let nums2 = vec![1, 3, 4, 2];
    println!(
        "[-1,3,-1] -> {:?}",
        Solution::next_greater_element(nums1, nums2)
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare_vector(output: &Vec<i32>, expected: &Vec<i32>) -> bool {
        if output.len() != expected.len() {
            return false;
        }
        for (o, e) in zip(output, expected) {
            if o != e {
                return false;
            }
        }
        true
    }

    #[test]
    fn tc_1() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let expected = vec![3, -1];
        let output = Solution::next_greater_element(nums1, nums2);
        dbg!(&output);
        assert!(compare_vector(&output, &expected));
    }

    #[test]
    fn tc_2() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let expected = vec![-1, 3, -1];
        let output = Solution::next_greater_element(nums1, nums2);
        dbg!(&output);
        assert!(compare_vector(&output, &expected));
    }
}
