struct Solution {}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let s_len = s.len();
        let mut count = 0;
        for word in &words {
            if word.len() <= s_len && s.starts_with(word) {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "ab".to_string(),
        "bc".to_string(),
        "abc".to_string(),
    ];
    let s = String::from("abc");
    println!("{}", Solution::count_prefixes(words, s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec!["a".to_string(), "a".to_string()];
        let s = String::from("aa");
        assert_eq!(Solution::count_prefixes(words, s), 2);

        let words = vec![
            "e".to_string(),
            "s".to_string(),
            "mi".to_string(),
            "e".to_string(),
            "ia".to_string(),
            "ibwu".to_string(),
            "e".to_string(),
            "e".to_string(),
            "k".to_string(),
            "ci".to_string(),
            "rip".to_string(),
            "suw".to_string(),
            "a".to_string(),
            "l".to_string(),
        ];
        let s = String::from("e");
        assert_eq!(Solution::count_prefixes(words, s), 4);

        let words = vec![
            "feh".to_string(),
            "w".to_string(),
            "w".to_string(),
            "lwd".to_string(),
            "c".to_string(),
            "s".to_string(),
            "vk".to_string(),
            "zwlv".to_string(),
            "n".to_string(),
            "w".to_string(),
            "sw".to_string(),
            "qrd".to_string(),
            "w".to_string(),
            "w".to_string(),
            "mqe".to_string(),
            "w".to_string(),
            "w".to_string(),
            "w".to_string(),
            "gb".to_string(),
            "w".to_string(),
            "qy".to_string(),
            "xs".to_string(),
            "br".to_string(),
            "w".to_string(),
            "rypg".to_string(),
            "wh".to_string(),
            "g".to_string(),
            "w".to_string(),
            "w".to_string(),
            "fh".to_string(),
            "w".to_string(),
            "w".to_string(),
            "sccy".to_string(),
        ];
        let s = String::from("w");
        assert_eq!(Solution::count_prefixes(words, s), 14);
    }
}
