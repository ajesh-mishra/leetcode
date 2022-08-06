struct Solution {
    q_row: String,
    h_row: String,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            q_row: "qwertyuiopQWERTYUIOP".to_string(),
            h_row: "asdfghjklASDFGHJKL".to_string(),
        }
    }

    pub fn find_words(self, words: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for word in words {
            let mut count: i32 = 0;
            for w in word.chars() {
                if self.q_row.contains(w) {
                    count += 1;
                } else if self.h_row.contains(w) {
                    count += 5;
                }
            }
            match count.abs() {
                0 => output.push(word),
                len => {
                    let word_len = word.len() as i32;
                    if len == word_len || len == 5 * word_len {
                        output.push(word)
                    }
                }
            }
        }
        output
    }
}

fn main() {
    let words = vec![
        String::from("Hello"),
        String::from("Alaska"),
        String::from("Dad"),
        String::from("Peace"),
    ];

    let s = Solution::new();
    println!("{:?}", s.find_words(words));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::zip;

    fn compare_vector(a: &Vec<String>, e: &Vec<String>) -> bool {
        if a.len() != e.len() {
            return false;
        }
        for (aw, ew) in zip(a, e) {
            if aw != ew {
                return false;
            }
        }
        true
    }

    #[test]
    fn tc_1() {
        let words = vec!["omk".to_string()];
        let s = Solution::new();
        let answer = s.find_words(words);
        println!("{:?}", answer);
        assert!(compare_vector(&answer, &vec![]));
    }

    #[test]
    fn tc_2() {
        let words = vec!["adsdf".to_string(), "sfd".to_string()];
        let s = Solution::new();
        let answer = s.find_words(words);
        assert!(compare_vector(
            &answer,
            &vec!["adsdf".to_string(), "sfd".to_string()]
        ));
    }

    #[test]
    fn tc_3() {
        let words = vec![
            String::from("Hello"),
            String::from("Alaska"),
            String::from("Dad"),
            String::from("Peace"),
        ];
        let s = Solution::new();
        let answer = s.find_words(words);
        assert!(compare_vector(
            &answer,
            &vec!["Alaska".to_string(), "Dad".to_string()]
        ));
    }
}
