use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        let words_len = words.len();

        for i in 0..words_len - 1 {
            for j in i + 1..words_len {
                let (is_substring, word) = if words[i].len() > words[j].len() {
                    (words[i].contains(&words[j]), &words[j])
                } else {
                    (words[j].contains(&words[i]), &words[i])
                };
                if is_substring {
                    result.insert(word);
                }
            }
        }

        result.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }
}

fn main() {
    let words = vec![
        "mass".to_string(),
        "as".to_string(),
        "hero".to_string(),
        "superhero".to_string(),
    ];
    println!("{:?}", Solution::string_matching(words));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec![
            "leetcoder".to_string(),
            "leetcode".to_string(),
            "od".to_string(),
            "hamlet".to_string(),
            "am".to_string(),
        ];
        assert_eq!(
            Solution::string_matching(words),
            vec!["leetcode".to_string(), "od".to_string(), "am".to_string()]
        );
    }
}
