struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut prefix = String::from("");
        let mut pos: Option<usize> = None;
        for (i, c) in word.char_indices() {
            prefix = format!("{c}{prefix}");
            if c == ch {
                pos = Some(i + 1);
                break;
            }
        }
        match pos {
            Some(x) => format!("{prefix}{}", &word[x..]),
            None => word,
        }
    }
}

fn main() {
    let word = String::from("abcdefd");
    println!("{}", Solution::reverse_prefix(word, 'd'));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let word = String::from("abcdefd");
        assert_eq!(Solution::reverse_prefix(word, 'd'), String::from("dcbaefd"));
        let word = String::from("xyxzxe");
        assert_eq!(Solution::reverse_prefix(word, 'z'), String::from("zxyxxe"));
        let word = String::from("abcd");
        assert_eq!(Solution::reverse_prefix(word, 'z'), String::from("abcd"));
    }
}
