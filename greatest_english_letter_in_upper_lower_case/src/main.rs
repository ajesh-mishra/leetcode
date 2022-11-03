#[derive(Debug, Copy, Clone)]
pub struct Case {
    upper: bool,
    lower: bool,
}

impl Case {
    pub fn new() -> Self {
        Case {
            upper: false,
            lower: false,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut letters = [Case::new(); 26];
        for c in s.chars() {
            match c as usize {
                x if x < 97 => letters[x - 65].upper = true,
                x => letters[x - 97].lower = true,
            }
        }
        for (i, case) in letters.iter().rev().enumerate() {
            if case.lower && case.upper {
                let pos = (25 - i as u32) + 65;
                return char::from_u32(pos).unwrap().to_string();
            }
        }
        String::from("")
    }
}

fn main() {
    let s = String::from("lEeTcOdE");
    println!("{}", Solution::greatest_letter(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_non_empty() {
        let s = String::from("arRAzFif");
        assert_eq!(Solution::greatest_letter(s), String::from("R"))
    }

    #[test]
    fn ut_empty() {
        let s = String::from("AbCdEfGhIjK");
        assert_eq!(Solution::greatest_letter(s), String::from(""))
    }
}
