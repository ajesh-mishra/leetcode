use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    fn finger_print(word: String) -> String {
        let mut fp_set = HashSet::new();
        for c in word.bytes() {
            fp_set.insert(c);
        }
        let mut fp_vec: Vec<u8> = fp_set.iter().map(|&x| x).collect();
        fp_vec.sort_by(|a, b| a.cmp(b));
        String::from_utf8(fp_vec).unwrap()
    }
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut pair_map = HashMap::new();
        for word in words {
            let fp = Self::finger_print(word);
            let count = pair_map.entry(fp).or_insert(0);
            *count += 1;
        }
        pair_map
            .values()
            .filter(|&&x| x > 1)
            .map(|&n| (n - 2 + 1..=n).product::<i32>() / 2)
            .sum::<i32>()
    }
}

fn main() {
    let words = vec![
        "aba".to_owned(),
        "aabb".to_owned(),
        "abcd".to_owned(),
        "bac".to_owned(),
        "aabc".to_owned(),
    ];
    println!("{}", Solution::similar_pairs(words));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let words = vec!["aabb".to_owned(), "ab".to_owned(), "ba".to_owned()];
        assert_eq!(Solution::similar_pairs(words), 3);
    }
}
