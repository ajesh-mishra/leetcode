use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        let mut count = i32::MIN;
        for value in strs {
            let v: i32 = value.parse().unwrap_or(value.len() as i32);
            count = max(count, v);
        }
        count
    }
}

fn main() {
    let strs = vec![
        "alic3".to_owned(),
        "bob".to_owned(),
        "3".to_owned(),
        "4".to_owned(),
        "00000".to_owned(),
    ];
    println!("{}", Solution::maximum_value(strs));
}
