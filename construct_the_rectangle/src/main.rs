struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut result = vec![area, 1];

        for i in 1..area {
            if area % i == 0 {
                let pair = area / i;

                if pair >= i {
                    result.clear();
                    result.push(pair);
                    result.push(i);
                } else {
                    break;
                }
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::construct_rectangle(4));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare_vector(result: &Vec<i32>, expected: &Vec<i32>) -> bool {
        if result.len() != expected.len() {
            return false;
        }

        for (r, e) in zip(result, expected) {
            if r != e {
                return false;
            }
        }

        true
    }

    #[test]
    fn tc_1() {
        let result = Solution::construct_rectangle(4);
        assert!(compare_vector(&result, &vec![2, 2]));
    }

    #[test]
    fn tc_2() {
        let result = Solution::construct_rectangle(122122);
        assert!(compare_vector(&result, &vec![427, 286]));
    }
}