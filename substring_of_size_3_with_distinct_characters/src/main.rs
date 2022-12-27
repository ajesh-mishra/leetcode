use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes()
            .windows(3)
            .filter(|sub| sub.iter().collect::<HashSet<&u8>>().len() == 3)
            .count() as _
    }
    pub fn count_good_substrings_1(s: String) -> i32 {
        let mut count = 0;
        for sub in s.into_bytes().windows(3) {
            let x: HashSet<&u8> = sub.iter().collect();
            if x.len() == 3 {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let s = String::from("aababcabc");
    println!("{}", Solution::count_good_substrings(s));
    let s = String::from("aababcabc");
    println!("{}", Solution::count_good_substrings_1(s));
}
