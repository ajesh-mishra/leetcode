use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn clean_paragraph(para: &str) -> HashMap<String, u32> {
        let mut para_map: HashMap<String, u32> = HashMap::new();

        let cleaned_para = para
            .chars()
            .filter_map(|c| match c {
                'a'..='z' => Some(c),
                'A'..='Z' => Some(c.to_ascii_lowercase()),
                ' ' | ',' => Some(' '),
                _ => None,
            })
            .fold(String::from(""), |acc, c| format!("{acc}{c}"));

        for word in cleaned_para.split_whitespace() {
            let count = para_map.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
        
        para_map
    }

    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let para_map = Solution::clean_paragraph(&paragraph);
        let mut max_occurance = u32::MIN;
        let mut common_word = String::from("");

        for (word, occcurance) in para_map {
            if occcurance > max_occurance && !banned.contains(&word.to_string()) {
                max_occurance = occcurance;
                common_word = word.to_string();
            }
        }

        common_word
    }
}

fn main() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned = vec!["hit".to_string()];
    println!("{}", Solution::most_common_word(paragraph, banned));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
        let banned = vec!["hit".to_string()];
        assert_eq!(
            Solution::most_common_word(paragraph, banned),
            String::from("ball")
        );

        let paragraph = "a.".to_string();
        let banned: Vec<String> = vec![];
        assert_eq!(
            Solution::most_common_word(paragraph, banned),
            String::from("a")
        );
    }
}
