use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let conversion = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut unique_words: HashSet<String> = HashSet::new();

        for word in words {
            let mut  morse_word = String::from("");
            for w in word.bytes() {
                morse_word.push_str(conversion[w as usize - 97]);
            }
            unique_words.insert(morse_word);
        }

        unique_words.len() as i32
    }
}

fn main() {
    let words = vec![
        "gin".to_string(),
        "zen".to_string(),
        "gig".to_string(),
        "msg".to_string(),
    ];
    println!("{}", Solution::unique_morse_representations(words));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec!["a".to_string()];
        assert_eq!(Solution::unique_morse_representations(words), 1);
    }
}