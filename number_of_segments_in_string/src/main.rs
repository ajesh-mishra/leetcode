struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        
        let mut counter = 0;
        let mut has_chars = false;
        let mut last_white_space = 0;
        
        for (i, c) in s.trim().chars().enumerate() {
            if c.is_whitespace() {
                if last_white_space + 1 != i || last_white_space == 0 {
                    counter += 1;
                }
                last_white_space = i;
            } else {
                has_chars = true;
            }
        }

        if has_chars {
            counter + 1
        } else {
            counter
        }
    }
}

fn main() {
    let s = String::from("Hello, my name is John");
    println!("{}", Solution::count_segments(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let s = String::from("Hello, my name is John");
        assert_eq!(Solution::count_segments(s), 5);
    }

    #[test]
    fn tc_2() {
        let s = String::from("Hello");
        assert_eq!(Solution::count_segments(s), 1);
    }

    #[test]
    fn tc_3() {
        let s = String::from(" Hello ");
        assert_eq!(Solution::count_segments(s), 1);
    }

    #[test]
    fn tc_4() {
        let s = String::from(", , , , a, eaefa");
        assert_eq!(Solution::count_segments(s), 6);
    }
}