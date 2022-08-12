struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let len = word.len();
        let mut first = 0;
        let mut rest = 0;

        if let Some(c) = word.chars().next() {
            first = if c.is_ascii_lowercase() {
                1
            } else {
                5
            }
        }

        for c in word[1..len].chars() {
            if c.is_ascii_lowercase() {
                rest += 1;
            } else if c.is_ascii_uppercase() {
                rest += 5;
            }
        }

        if rest == (len - 1) {
            return true;
        } else if first == 5 && rest == (len - 1) * 5 {
            return true;
        }

        false
    }
}

fn main() {
    let word = String::from("USA");
    println!("{}", Solution::detect_capital_use(word));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_all_caps() {
        let word = String::from("USA");
        assert!(Solution::detect_capital_use(word));
        let word = String::from("INDIA");
        assert!(Solution::detect_capital_use(word));
    }

    #[test]
    fn ut_all_lower() {
        let word = String::from("usa");
        assert!(Solution::detect_capital_use(word));
        let word = String::from("india");
        assert!(Solution::detect_capital_use(word));
    }

    #[test]
    fn ut_init_cap() {
        let word = String::from("Ajesh");
        assert!(Solution::detect_capital_use(word));
        let word = String::from("Mishra");
        assert!(Solution::detect_capital_use(word));
    }

    #[test]
    fn ut_negetive() {
        let word = String::from("ajEsh");
        assert!(!Solution::detect_capital_use(word));
        let word = String::from("mISHRA");
        assert!(!Solution::detect_capital_use(word));
        let word = String::from("mIsHrA");
        assert!(!Solution::detect_capital_use(word));
    }
}
