use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut non_alpha: HashMap<usize, char> = HashMap::new();
        let mut only_alpha = String::from("");
        let mut result = String::from("");

        for (i, c) in s.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' => only_alpha.push(c),
                _ => {
                    non_alpha.insert(i, c);
                }
            }
        }

        let mut x = only_alpha.chars();

        for i in 0..s.len() {
            if let Some(&x) = non_alpha.get(&i) {
                result.push(x);
            } else {
                result.push(x.next_back().unwrap());
            }
        }

        result
    }
}

fn main() {
    let s = String::from("ab-cd");
    println!("{}", Solution::reverse_only_letters(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("a-bC-dEf-ghIj");
        assert_eq!(
            Solution::reverse_only_letters(s),
            String::from("j-Ih-gfE-dCba")
        );

        let s = String::from("Test1ng-Leet=code-Q!");
        assert_eq!(
            Solution::reverse_only_letters(s),
            String::from("Qedo1ct-eeLg=ntse-T!")
        );
    }

    #[test]
    fn ut_2() {
        let s = String::from("-");
        assert_eq!(Solution::reverse_only_letters(s), String::from("-"));

        let s = String::from("z<*zj");
        assert_eq!(Solution::reverse_only_letters(s), String::from("j<*zz"));
    }
}
