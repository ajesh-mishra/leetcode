struct Solution {}

impl Solution {
    fn is_palindrome(substring: &str) -> bool {
        let mut i = 0;
        let mut j = substring.len();
        while i <= j {
            if substring[i..i+1] != substring[j-1..j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
    pub fn longest_palindrome(s: String) -> String {
        for win in (1..s.len() + 1).rev() {
            let mut init = 0;
            while init + win <= s.len() {
                let substring = &s[init..init + win];
                if Self::is_palindrome(substring) {
                    return substring.to_string();
                }
                init += 1;
            }
        }
        String::from("")
    }
}

fn main() {
    let s = String::from("babad");
    println!("{}", Solution::longest_palindrome(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_longest_palindrome() {
        let s = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome(s), String::from("bb"));
        let s = String::from("a");
        assert_eq!(Solution::longest_palindrome(s), String::from("a"));
    }

    #[test]
    fn ut_is_palindrome() {
        // true scenarios
        assert!(Solution::is_palindrome("aba"));
        assert!(Solution::is_palindrome("aa"));
        assert!(Solution::is_palindrome("abba"));
        assert!(Solution::is_palindrome("a1b2b1a"));
        assert!(Solution::is_palindrome("a1bb1a"));
        
        // false scenarios
        assert!(!Solution::is_palindrome("ab"));
        assert!(!Solution::is_palindrome("acba"));
        assert!(!Solution::is_palindrome("a12b1a"));
    }
}