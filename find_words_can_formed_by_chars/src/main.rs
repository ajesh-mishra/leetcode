struct Solution {}

impl Solution {
    fn get_arr(word: &str) -> [i32; 26] {
        let mut char_arr = [0; 26];
        for c in word.chars() {
            let pos = (c as u8 - 97) as usize;
            char_arr[pos] += 1;
        }
        char_arr
    }
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut count = 0;
        let chars_arr = Self::get_arr(&chars);
        for word in words {
            let mut can_form = true;
            let word_arr = Self::get_arr(&word);
            for i in 0..26 {
                if chars_arr[i] < word_arr[i] {
                    can_form = false;
                    break;
                }
            }
            if can_form {
                count += word.len() as i32;
            }
        }
        count
    }
}

fn main() {
    let words = vec![
        "cat".to_string(),
        "bt".to_string(),
        "hat".to_string(),
        "tree".to_string(),
    ];
    let chars = String::from("aztach");
    println!("{}", Solution::count_characters(words, chars));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ];
        let chars = String::from("welldonehoneyr");
        assert_eq!(Solution::count_characters(words, chars), 10);
    }
}
