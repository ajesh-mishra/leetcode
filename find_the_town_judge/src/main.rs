use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 && trust.is_empty() {
            return 1;
        }
        let mut occurance: HashMap<i32, u8> = HashMap::new();
        let mut trusters: HashSet<i32> = HashSet::new();
        for mut t in trust {
            let y = t.pop().unwrap();
            let count = occurance.entry(y).or_insert(0);
            *count += 1;
            let x = t.pop().unwrap();
            trusters.insert(x);
        }
        let total = match n % 2 {
            0 => (n / 2) * (n + 1),
            _ => n * ((n + 1) / 2),
        };
        let calculated_total: i32 = trusters.into_iter().sum();
        let diff = total - calculated_total;
        if let Some(x) = occurance.get(&diff) {
            if x + 1 == n as u8 {
                return diff;
            }
        }
        -1
    }
}

fn main() {
    let trust = vec![vec![1, 3], vec![2, 3]];
    println!("{}", Solution::find_judge(3, trust));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let trust = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::find_judge(3, trust), -1);
    }
}
