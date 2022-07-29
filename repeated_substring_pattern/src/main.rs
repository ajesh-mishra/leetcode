struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();

        for i in 1..=(len / 2) {
            if len % i == 0 {
                let substr = &s[0..i];
                let mut calculated = String::from(substr);

                for _ in 1..(len / i) {
                    calculated.push_str(substr);
                }

                if calculated == s {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let s = String::from("abab");
    println!("{}", Solution::repeated_substring_pattern(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let s = String::from("abab");
        assert!(Solution::repeated_substring_pattern(s));
    }

    #[test]
    fn tc_2() {
        let s = String::from("aba");
        assert!(!Solution::repeated_substring_pattern(s));
    }

    #[test]
    fn tc_3() {
        let s = String::from("abcabcabcabc");
        assert!(Solution::repeated_substring_pattern(s));
    }
}
