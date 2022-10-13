use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut ordered_nums = nums.clone();
        let mut memo: HashMap<i32, i32> = HashMap::new();
        ordered_nums.sort_by(|a, b| a.cmp(b));
        for i in nums {
            let pos = match memo.get(&i) {
                Some(&p) => p,
                None => {
                    let p = ordered_nums.iter().position(|&x| x == i).unwrap() as i32;
                    memo.insert(i, p);
                    p
                }
            };
            result.push(pos.clone());
        }
        result
    }
}

fn main() {
    let nums = vec![8, 1, 2, 2, 3];
    println!("{:?}", Solution::smaller_numbers_than_current(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![6, 5, 4, 8];
        assert_eq!(
            Solution::smaller_numbers_than_current(nums),
            vec![2, 1, 0, 3]
        );

        let nums = vec![7, 7, 7, 7];
        assert_eq!(
            Solution::smaller_numbers_than_current(nums),
            vec![0, 0, 0, 0]
        );
    }
}
