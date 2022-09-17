use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s == goal {
            let y: HashSet<char> = HashSet::from_iter(s.chars());
            if s.len() == y.len() {
                return false;
            }
        }

        let mut s_str = String::from("");
        let mut first_mismatch = String::from("");
        let mut second_mismatch = String::from("");
        let mut count: usize = 0;

        for (s_char, g_char) in s.chars().zip(goal.chars()) {
            match s_char == g_char {
                false if count < 2 => {
                    s_str.push(g_char);
                    if count == 0 {
                        first_mismatch = format!("{s_char}{g_char}");
                    } else {
                        second_mismatch = format!("{g_char}{s_char}");
                    };
                    count += 1;
                }
                _ => {
                    s_str.push(s_char);
                }
            }
        }

        goal == s_str && first_mismatch == second_mismatch
    }
}

fn main() {
    let s = String::from("abbbbxyz");
    let goal = String::from("xyzaaba");
    println!("{}", Solution::buddy_strings(s, goal));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let s = String::from("ab");
        let goal = String::from("ba");
        assert!(Solution::buddy_strings(s, goal));

        let s = String::from("aa");
        let goal = String::from("aa");
        assert!(Solution::buddy_strings(s, goal));

        let s = String::from("abc");
        let goal = String::from("cba");
        assert!(Solution::buddy_strings(s, goal));
    }

    #[test]
    fn ut_false() {
        let s = String::from("ab");
        let goal = String::from("ab");
        assert!(!Solution::buddy_strings(s, goal));

        let s = String::from("abcd");
        let goal = String::from("badc");
        assert!(!Solution::buddy_strings(s, goal));

        let s = String::from("abcaa");
        let goal = String::from("abcbb");
        assert!(!Solution::buddy_strings(s, goal));
    }
}
