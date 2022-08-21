struct Solution {}

impl Solution {
    // Using Iterators
    pub fn valid_palindrome_1(s: String) -> bool {
        let r: String = s.chars().rev().collect();
        let mut count: usize = 0;
        let len = s.len();
        let max = (len as i32 / 2) + 1;
        let mut is = s.chars();
        let mut ir = s.chars().rev();

        while let (Some(sc), Some(rc)) = (is.next(), ir.next()) {
            if count as i32 >= max {
                break;
            }

            if sc != rc {
                if s[count..len - count - 1] == r[count + 1..len - count] {
                    break;
                } else if s[count + 1..len - count] == r[count..len - count - 1] {
                    break;
                } else {
                    return false;
                }
            }

            count += 1;
        }

        true
    }

    // Using Zip Function
    pub fn valid_palindrome_2(s: String) -> bool {
        use std::iter::zip;
        let r: String = s.chars().rev().collect();
        let mut count = 0;
        let len = s.len();
        let max = (len as i32 / 2) + 1;

        for (sc, rc) in zip(s.chars(), r.chars()) {
            if count as i32 >= max {
                break;
            }

            if sc != rc {
                if s[count..len - count - 1] == r[count + 1..len - count] {
                    break;
                } else if s[count + 1..len - count] == r[count..len - count - 1] {
                    break;
                } else {
                    return false;
                }
            }

            count += 1;
        }

        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let sb = s.as_bytes();
        let mut num_drop = 0;
        let (mut i, mut j) = (0, s.len() - 1);
        let (mut i_drop, mut j_drop) = (0, 0);
        while i < j {
            if sb[i] != sb[j] {
                if num_drop == 2 {
                    return false;
                } else if num_drop == 0 {
                    i_drop = i;
                    j_drop = j;
                    i += 1;
                } else {
                    i = i_drop;
                    j = j_drop;
                    j -= 1;
                }
                num_drop += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}

fn main() {
    let s = String::from("atbucuba");
    println!("{}", Solution::valid_palindrome_1(s));
    
    let s = String::from("atbucuba");
    println!("{}", Solution::valid_palindrome_2(s));

    let s = String::from("atbucuba");
    println!("{}", Solution::valid_palindrome(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_success() {
        let s = String::from("abucubta");
        assert!(Solution::valid_palindrome(s));
        let s = String::from("abucuba");
        assert!(Solution::valid_palindrome(s));
        let s = String::from("abuuba");
        assert!(Solution::valid_palindrome(s));
        let s = String::from("abuctuba");
        assert!(Solution::valid_palindrome(s));
        let s = String::from("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga");
        assert!(Solution::valid_palindrome(s));
    }

    #[test]
    fn ut_failure() {
        let s = String::from("abc");
        assert!(!Solution::valid_palindrome(s));
        let s = String::from("abc");
        assert!(!Solution::valid_palindrome(s));
    }
}
