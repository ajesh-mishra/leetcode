use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut ls = s.len();
        let mut str_it = s.chars();

        for word in words.into_iter() {
            let lw = word.len();
            if ls < lw
                || !str_it
                    .by_ref()
                    .take(lw)
                    .zip(word.chars())
                    .all(|(c1, c2)| c1 == c2)
            {
                return false;
            } else if ls == lw {
                return true;
            }
            ls -= lw;
        }
        ls == 0
    }
    pub fn is_prefix_string_1(s: String, words: Vec<String>) -> bool {
        let s_len = s.len();
        let mut n_len = 0;
        let mut new_s = String::from("");
        for word in words {
            n_len += word.len();
            new_s.push_str(word.as_str());
            match n_len.cmp(&s_len) {
                Ordering::Equal => break,
                Ordering::Greater => return false,
                Ordering::Less => continue,
            }
        }
        s == new_s
    }
}

fn main() {
    let s = String::from("iloveleetcode");
    let words = vec![
        "i".to_owned(),
        "love".to_owned(),
        "leetcode".to_owned(),
        "apples".to_owned(),
    ];
    println!(
        "{}",
        Solution::is_prefix_string(s.to_owned(), words.clone())
    );
    println!("{}", Solution::is_prefix_string_1(s, words));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let s = String::from("iloveleetcode");
        let words = vec![
            "i".to_owned(),
            "love".to_owned(),
            "leetcode".to_owned(),
            "apples".to_owned(),
        ];
        assert!(Solution::is_prefix_string(s, words));
    }

    #[test]
    fn ut_false() {
        let s = String::from("iloveleetcode");
        let words = vec![
            "apples".to_owned(),
            "i".to_owned(),
            "love".to_owned(),
            "leetcode".to_owned(),
        ];
        assert!(!Solution::is_prefix_string(s, words));
    }
}
