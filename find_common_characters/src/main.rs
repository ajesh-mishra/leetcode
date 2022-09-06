use std::cmp::min;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn mapping(word: &str) -> HashMap<char, i8> {
        let mut map: HashMap<char, i8> = HashMap::new();
        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        map
    }

    pub fn common_chars(mut words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut keys = Vec::new();
        let last = words.pop().unwrap();
        let mut last_map = Solution::mapping(&last);

        for &key in last_map.keys() {
            keys.push(key);
        }

        for word in words {
            let map = Solution::mapping(&word);
            for &k in &keys {
                let y = map.get(&k).unwrap_or(&0);
                let x = last_map.get(&k).unwrap();
                last_map.insert(k, min(*x, *y));
            }
        }

        for (k, v) in last_map {
            for _ in 0..v {
                result.push(k.to_string());
            }
        }

        result
    }
}

fn main() {
    let words = vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string(),
    ];
    println!("{:?}", Solution::common_chars(words));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        assert_eq!(
            Solution::common_chars(words),
            vec!["c".to_string(), "o".to_string()]
        );
    }
}
