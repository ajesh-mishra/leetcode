use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for task in tasks {
            map.entry(task).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut result = 0;
        for &i in map.values() {
            let (res, rem) = (i / 3, i % 3);
            result += match (res, rem) {
                (0, 1) => return -1,
                (res, 0) => res,
                (res, _) => res + 1,
            }
        }
        result
    }
}

fn main() {
    let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
    println!("{}", Solution::minimum_rounds(tasks));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_positive() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        assert_eq!(Solution::minimum_rounds(tasks), 4);
    }

    #[test]
    fn ut_negetive() {
        let tasks = vec![2, 3, 3];
        assert_eq!(Solution::minimum_rounds(tasks), -1);
    }
}
