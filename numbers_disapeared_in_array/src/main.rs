use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut max: i32 = 0;

        if let Some(m) = num_set.clone().into_iter().max() {
            max = m.clone();
        }

        let exp_set: HashSet<i32> = (1..=max).map(|v| v).collect();
        let mut result = Vec::new();

        for &i in exp_set.difference(&num_set) {
            result.push(i);
        }

        if max == nums_len as i32 {
            return result;
        }

        let end = (nums_len - num_set.len() - result.len()) as i32;
        let temp = (max + 1..=max + end).map(|v| v).collect::<Vec<i32>>();

        for i in temp {
            result.push(i);
        }

        result
    }
}

fn main() {
    let input = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", Solution::find_disappeared_numbers(input));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare(output: &Vec<i32>, expected: &Vec<i32>) -> bool {
        if output.len() != expected.len() {
            return false;
        }

        for (o, e) in zip(output, expected) {
            if o != e {
                println!("{} - {}", o, e);
                return false;
            }
        }

        true
    }

    #[test]
    fn tc_1() {
        let input = vec![1, 1];
        let expected = vec![2];
        let output = Solution::find_disappeared_numbers(input);
        assert!(compare(&output, &expected));
    }

    #[test]
    fn tc_2() {
        let input = vec![1, 1, 2, 2];
        let expected = vec![3, 4];
        let output = Solution::find_disappeared_numbers(input);
        assert!(compare(&output, &expected));
    }

    #[test]
    fn tc_3() {
        let input = vec![4, 3, 2, 7, 7, 2, 3, 1];
        let expected = vec![5, 6, 8];
        let output = Solution::find_disappeared_numbers(input);
        println!("{:?}", output);
        assert!(compare(&output, &expected));
    }
}
