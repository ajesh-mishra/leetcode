struct Solution {}

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut zeros = 0;
        let mut prev = None;
        let mut result = vec![];
        let mut check_and_push = |num: i32| match num {
            0 => zeros += 1,
            _ => result.push(num),
        };
        for slice in nums.windows(2) {
            if let Some(p) = prev {
                if p == slice[1] {
                    check_and_push(p * 2);
                    prev = Some(0);
                } else {
                    check_and_push(p);
                    prev = Some(slice[1]);
                }
            } else {
                if slice[0] == slice[1] {
                    check_and_push(slice[0] * 2);
                    prev = Some(0);
                } else {
                    check_and_push(slice[0]);
                    prev = Some(slice[1]);
                }
            }
        }
        if let Some(p) = prev {
            result.push(p);
        }
        for _ in 0..zeros {
            result.push(0);
        }
        result
    }
}

fn main() {
    let nums = vec![1, 2, 2, 1, 1, 0];
    println!("{:?}", Solution::apply_operations(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![0, 1];
        assert_eq!(Solution::apply_operations(nums), vec![1, 0]);
    }
}
