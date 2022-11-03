use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut map = HashMap::new();
        for word in words {
            let mut diff = Vec::with_capacity(2);
            for (a, b) in word.chars().zip(word.chars().skip(1)) {
                let d = b as i32 - a as i32;
                diff.push(d);
            }
            let (count, _) = map.entry(diff).or_insert((0, word));
            *count += 1;
            if *count > 1 && map.len() > 1 {
                break;
            }
        }
        for (_, (count, word)) in map {
            if count == 1 {
                return word;
            }
        }
        unreachable!()
    }
}

fn main() {
    let words = vec!["adc".to_string(), "wzy".to_string(), "abc".to_string()];
    println!("{}", Solution::odd_string(words));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let words = vec![
            "aaa".to_string(),
            "bob".to_string(),
            "ccc".to_string(),
            "ddd".to_string(),
        ];
        assert_eq!(Solution::odd_string(words), String::from("bob"));
    }

    #[test]
    fn ut_2() {
        let words = vec![
            "ddd".to_string(),
            "poo".to_string(),
            "baa".to_string(),
            "onn".to_string(),
        ];
        assert_eq!(Solution::odd_string(words), String::from("ddd"));
    }
}
