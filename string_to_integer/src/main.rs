struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut result = String::from("");
        let mut has_number = false;
        let mut is_negetive = false;
        let mut is_positive = false;
        for c in s.chars() {
            match c {
                ' ' if (is_negetive || is_positive) && !has_number => return 0,
                ' ' if !has_number => {}
                '+' if !has_number && !is_positive => is_positive = true,
                '-' if !has_number && !is_negetive => is_negetive = true,
                '0'..='9' => {
                    result.push(c);
                    has_number = true;
                }
                _ if has_number => break,
                _ => return 0,
            }
        }
        if !has_number || (is_positive && is_negetive) {
            return 0;
        }
        match (is_negetive, result.parse::<i32>()) {
            (true, Ok(x)) => -1 * x,
            (false, Ok(x)) => x,
            (true, Err(_)) => i32::MIN,
            (false, Err(_)) => i32::MAX,
        }
    }
}

fn main() {
    let s = String::from("42");
    println!("{}", Solution::my_atoi(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_non_zero() {
        let s = String::from("    -42");
        assert_eq!(Solution::my_atoi(s), -42);
        let s = String::from("4193 with words");
        assert_eq!(Solution::my_atoi(s), 4193);
        let s = String::from("+1");
        assert_eq!(Solution::my_atoi(s), 1);
    }

    #[test]
    fn ut_zero() {
        let s = String::from("words and 987");
        assert_eq!(Solution::my_atoi(s), 0);
        let s = String::from("_");
        assert_eq!(Solution::my_atoi(s), 0);
        let s = String::from("+-12");
        assert_eq!(Solution::my_atoi(s), 0);
        let s = String::from("  +  413");
        assert_eq!(Solution::my_atoi(s), 0);
    }
}
