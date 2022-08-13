struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::from("");
        for (i, word) in s.split_whitespace().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            for c in word.chars().rev() {
                result.push(c);
            }
        }
        result
    }
}

fn main() {
    let s = String::from("Let's take LeetCode contest");
    println!("{}", Solution::reverse_words(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("God Ding");
        assert_eq!(Solution::reverse_words(s), String::from("doG gniD"));
        let s = String::from("Let's take LeetCode contest");
        assert_eq!(Solution::reverse_words(s), String::from("s'teL ekat edoCteeL tsetnoc"));
    }
}
