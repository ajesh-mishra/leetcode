use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_vec: Vec<char> = Vec::new();
        let mut result: usize = 0;

        for c in s.chars() {
            if char_vec.contains(&c) {
                result = max(result, char_vec.len());
                let pos = char_vec.iter().position(|&x| x == c).unwrap();
                char_vec.drain(..pos + 1);
            }
            char_vec.push(c);
        }

        max(result, char_vec.len()) as _
    }
}

fn main() {
    let s = String::from("abcabcbb");
    println!("{}", Solution::length_of_longest_substring(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("jlygy");
        assert_eq!(Solution::length_of_longest_substring(s), 4);
        let s = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        let s = String::from("dvdf");
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        let s = String::from("bbbbbbb");
        assert_eq!(Solution::length_of_longest_substring(s), 1);
        let s = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        let s = String::from(" ");
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }
}
