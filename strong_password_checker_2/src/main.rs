struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let mut checks = [false; 4];
        let mut prev_char = None;
        for c in password.chars() {
            match c {
                'a'..='z' => checks[0] = true,
                'A'..='Z' => checks[1] = true,
                '0'..='9' => checks[2] = true,
                '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' => {
                    checks[3] = true
                }
                _ => {}
            }
            match prev_char {
                Some(p) if p == c => return false,
                _ => prev_char = Some(c),
            }
        }
        checks.iter().all(|&c| c)
    }
}

fn main() {
    let password = String::from("IloveLe3tcode!");
    println!("{}", Solution::strong_password_checker_ii(password));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let password = String::from("IloveLe3tcode!");
        assert!(Solution::strong_password_checker_ii(password));
    }

    #[test]
    fn ut_false() {
        let password = String::from("Me+You--IsMyDream");
        assert!(!Solution::strong_password_checker_ii(password));
        let password = String::from("1aB!");
        assert!(!Solution::strong_password_checker_ii(password));
    }
}
