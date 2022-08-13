struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let len = s.len();
        let step = k as usize;
        let mut result = String::from("");

        for (i, n) in (0..len).step_by(step).enumerate() {
            let end = if n + step > len { len } else { n + step };

            if i % 2 == 0 {
                for c in &mut s[n..end].chars().rev() {
                    result.push(c);
                }
            } else {
                result.push_str(&s[n..end]);
            }
        }

        result
    }
}

fn main() {
    let s = String::from("abcdefg");
    let k = 2;
    println!("{}", Solution::reverse_str(s, k));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("abcdefg");
        let k = 2;
        assert_eq!(Solution::reverse_str(s, k), String::from("bacdfeg"));
    }

    #[test]
    fn ut_2() {
        let s = String::from("abcd");
        let k = 2;
        assert_eq!(Solution::reverse_str(s, k), String::from("bacd"));
    }
}
