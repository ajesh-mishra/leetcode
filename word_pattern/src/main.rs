use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut s_count = 0;
        let mut s_print = String::from("");
        let mut s_map = HashMap::new();
        for c in s.split_whitespace() {
            let pos = s_map.entry(c).or_insert(s_count);
            s_print.push_str(format!("-{}", *pos).as_str());
            s_count += 1;
        }

        let mut p_count = 0;
        let mut p_print = String::from("");
        let mut p_map = HashMap::new();
        for c in pattern.chars() {
            let pos = p_map.entry(c).or_insert(p_count);
            p_print.push_str(format!("-{}", *pos).as_str());
            p_count += 1;
        }

        s_print == p_print
    }
}

fn main() {
    let pattern = String::from("abba");
    let s = String::from("dog cat cat dog");
    println!("{}", Solution::word_pattern(pattern, s));
}
