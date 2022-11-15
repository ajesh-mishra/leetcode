struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut result = String::from("");
        for (i, word) in title.split(" ").enumerate() {
            if i != 0 {
                result.push(' ');
            }
            if word.len() > 2 {
                let mut init_cap_word = String::from("");
                for (i, c) in word.char_indices() {
                    if i == 0 {
                        init_cap_word.push(c.to_ascii_uppercase());
                        continue;
                    }
                    init_cap_word.push(c.to_ascii_lowercase());
                }
                result.push_str(&init_cap_word);
            } else {
                let mut lower_word = String::from("");
                for c in word.chars() {
                    lower_word.push(c.to_ascii_lowercase());
                }
                result.push_str(&lower_word);
            }
        }
        result
    }
}

fn main() {
    let title = String::from("capiTalIze tHe titLe");
    println!("{}", Solution::capitalize_title(title));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let title = String::from("First leTTeR of EACH Word");
        assert_eq!(
            Solution::capitalize_title(title),
            String::from("First Letter of Each Word")
        );

        let title = String::from("i lOve leetcode");
        assert_eq!(
            Solution::capitalize_title(title),
            String::from("i Love Leetcode")
        );
    }
}
