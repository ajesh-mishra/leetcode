struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s1 = Vec::new();
        let mut t1 = Vec::new();

        for c in s.chars() {    
            if c == '#' {
                s1.pop();
            } else {
                s1.push(c)
            }
        }

        for c in t.chars() {    
            if c == '#' {
                t1.pop();
            } else {
                t1.push(c)
            }
        }
        
        s1 == t1
    }
}

fn main() {
    let s = "ab#c".to_string();
    let t = "ad#c".to_string();
    println!("{}", Solution::backspace_compare(s, t));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        assert!(Solution::backspace_compare(s, t));
    }

    #[test]
    fn ut_false() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        assert!(!Solution::backspace_compare(s, t));
    }
}