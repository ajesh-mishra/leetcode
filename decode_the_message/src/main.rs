use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let seq = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut seq_pos = 0;
        let mut table = HashMap::new();
        for c in key.chars() {
            if c == ' ' {
                continue;
            }
            if let None = table.get(&c) {
                table.insert(c, seq[seq_pos]);
                seq_pos += 1;
            }
        }
        let mut result = String::from("");
        for c in message.chars() {
            if c == ' ' {
                result.push(' ');
                continue;
            }
            if let Some(&x) = table.get(&c) {
                result.push(x);
            }
        }
        result
    }
}

fn main() {
    let key = String::from("the quick brown fox jumps over the lazy dog");
    let message = String::from("vkbs bs t suepuv");
    println!("{}", Solution::decode_message(key, message));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let key = String::from("eljuxhpwnyrdgtqkviszcfmabo");
        let message = String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb");
        assert_eq!(
            Solution::decode_message(key, message),
            String::from("the five boxing wizards jump quickly")
        );
    }
}
