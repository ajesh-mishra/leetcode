use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut result = HashMap::new();
        let mut output = String::from("");
        for word in s.split_whitespace() {
            let mut word = word.to_owned();
            let pos = word.pop().unwrap();
            result.entry(pos).or_insert(word);
        }
        for i in '1'..='9' {
            if let Some(x) = result.get(&i) {
                match i {
                    '1' => output.push_str(x),
                    _ => {
                        output.push(' ');
                        output.push_str(x);
                    }
                }
            }
        }
        output
    }
}

fn main() {
    let s = String::from("is2 sentence4 This1 a3");
    println!("{}", Solution::sort_sentence(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("Myself2 Me1 I4 and3");
        assert_eq!(Solution::sort_sentence(s), String::from("Me Myself and I"));
    }
}
