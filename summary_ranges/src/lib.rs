pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        } else if nums.len() == 1 {
            return vec![format!("{}", nums[0])];
        }

        let mut result: Vec<String> = Vec::new();
        let mut temp: Vec<i32> = Vec::new();

        for i in nums {
            let len = temp.len();

            if temp.is_empty() || temp[len - 1] + 1 == i {
                temp.push(i);
                continue;
            }

            if len == 1 {
                result.push(format!("{}", temp[0]));
            } else {
                result.push(format!("{}->{}", temp[0], temp[len - 1]));
            }

            temp = vec![i];
        }

        let len = temp.len();

        if len == 1 {
            result.push(format!("{}", temp[0]));
        } else {
            result.push(format!("{}->{}", temp[0], temp[len - 1]));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare_vector(output: &Vec<String>, expected: &Vec<String>) -> bool {
        if output.len() != expected.len() {
            return false;
        }

        for (o, e) in zip(output, expected) {
            println!("{}\t{}", o, e);
            if o != e {
                return false;
            }
        }

        true
    }

    #[test]
    fn tc_1() {
        let input = vec![0, 1, 2, 4, 5, 7];
        let output = Solution::summary_ranges(input);
        let expected = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];
        assert!(compare_vector(&output, &expected))
    }

    #[test]
    fn tc_2() {
        let input = vec![0, 2, 3, 4, 6, 8, 9];
        let output = Solution::summary_ranges(input);
        let expected = vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string(),
        ];
        assert!(compare_vector(&output, &expected))
    }

    #[test]
    fn tc_3() {
        let input = vec![-2147483648, -2147483647, 2147483647];
        let output = Solution::summary_ranges(input);
        let expected = vec![
            format!("{}->{}", -2147483648, -2147483647),
            "2147483647".to_string(),
        ];
        assert!(compare_vector(&output, &expected))
    }
}
