struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;
        for word in words {
            if word.starts_with(pref.as_str()) {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let words = vec![
        "pay".to_string(),
        "attention".to_string(),
        "practice".to_string(),
        "attend".to_string(),
    ];
    let pref = "at".to_string();
    println!("{}", Solution::prefix_count(words, pref));
}
