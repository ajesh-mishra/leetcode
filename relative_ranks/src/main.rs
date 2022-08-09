use std::collections::{BinaryHeap, HashMap};

struct Solution {}

impl Solution {
    pub fn find_relative_ranks_1(score: Vec<i32>) -> Vec<String> {
        let len = score.len();
        let mut rank = score.clone();
        let mut result = Vec::new();

        let podium_str: HashMap<usize, &str> =
            HashMap::from([(0, "Gold Medal"), (1, "Silver Medal"), (2, "Bronze Medal")]);

        for i in 0..len {
            for j in 0..(len - i - 1) {
                if rank[j] < rank[j + 1] {
                    let temp = rank[j + 1];
                    rank[j + 1] = rank[j];
                    rank[j] = temp;
                }
            }
        }

        for s in score {
            let pos = rank.iter().position(|&x| x == s).unwrap();

            if pos < 3 {
                if let Some(x) = podium_str.get(&pos) {
                    result.push(x.to_string());
                }
            } else {
                result.push(format!("{}", pos + 1));
            }
        }

        result
    }

    fn index_to_rank(i: usize) -> String {
        match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => (i + 1).to_string(),
        }
    }

    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.iter().fold(BinaryHeap::new(), |mut acc, s| {
            acc.push(s);
            acc
        });

        let ranks = (0..score.len()).fold(HashMap::new(), |mut acc, i| {
            acc.insert(sorted.pop().unwrap(), Self::index_to_rank(i));
            acc
        });

        score
            .iter()
            .map(|s| ranks.get(s).unwrap())
            .cloned()
            .collect()
    }
}

fn main() {
    let score = vec![5, 4, 3, 2, 1];
    println!("{:?}", Solution::find_relative_ranks_1(score.clone()));
    println!("{:?}", Solution::find_relative_ranks(score));
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
            if o != e {
                return false;
            }
        }
        true
    }

    #[test]
    fn tc_1() {
        let score = vec![5, 4, 3, 2, 1];
        let output = Solution::find_relative_ranks(score);
        let expected = vec![
            "Gold Medal".to_string(),
            "Silver Medal".to_string(),
            "Bronze Medal".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];
        println!("{:?}", output);
        assert!(compare_vector(&output, &expected));
    }

    #[test]
    fn tc_2() {
        let score = vec![10, 3, 8, 9, 4];
        let output = Solution::find_relative_ranks(score);
        let expected = vec![
            "Gold Medal".to_string(),
            "5".to_string(),
            "Bronze Medal".to_string(),
            "Silver Medal".to_string(),
            "4".to_string(),
        ];
        println!("{:?}", output);
        assert!(compare_vector(&output, &expected));
    }
}
