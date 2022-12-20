use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn insert_unique(unique: &mut HashSet<String>, x: String) {
        let trimed_x = x.trim_start_matches('0').to_owned();
        match trimed_x.len() {
            0 => unique.insert(String::from("0")),
            _ => unique.insert(trimed_x),
        };
    }
    pub fn num_different_integers(word: String) -> i32 {
        let mut unique = HashSet::new();
        let mut temp: Option<String> = None;
        for c in word.chars() {
            match c {
                '0'..='9' => match temp {
                    Some(x) => temp = Some(format!("{x}{c}")),
                    None => temp = Some(format!("{c}")),
                },
                _ => {
                    if let Some(x) = temp {
                        Self::insert_unique(&mut unique, x);
                        temp = None;
                    }
                }
            }
        }
        if let Some(x) = temp {
            Self::insert_unique(&mut unique, x);
        }
        unique.len() as _
    }
}

fn main() {
    let word = String::from("a123bc34d8ef34");
    println!("{}", Solution::num_different_integers(word));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let word = String::from("leet1234code234");
        assert_eq!(Solution::num_different_integers(word), 2);
        let word = String::from("a1b01c001");
        assert_eq!(Solution::num_different_integers(word), 1);
        let word = String::from("4");
        assert_eq!(Solution::num_different_integers(word), 1);
        let word = String::from("0a0");
        assert_eq!(Solution::num_different_integers(word), 1);
    }
}
