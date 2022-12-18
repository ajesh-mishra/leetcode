use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut result = HashMap::new();
        let mut pos = 0;
        let mut forward = true;
        for c in s.chars() {
            let zig = result.entry(pos).or_insert("".to_owned());
            zig.push(c);
            pos += match forward {
                true if pos + 1 == num_rows => {
                    forward = false;
                    -1
                }
                false if pos == 0 => {
                    forward = true;
                    1
                }
                true => 1,
                false => -1,
            }
        }
        let mut result_str = String::from("");
        for i in 0..num_rows {
            if let Some(x) = result.get(&i) {
                result_str.push_str(x);
            }
        }
        result_str
    }
}

fn main() {
    let s = String::from("PAYPALISHIRING");
    println!("{}", Solution::convert(s, 3));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("PAYPALISHIRING");
        assert_eq!(Solution::convert(s, 4), String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn ut_2() {
        let s = String::from("A");
        assert_eq!(Solution::convert(s, 1), String::from("A"));
    }
}
