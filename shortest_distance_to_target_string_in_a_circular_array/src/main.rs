use std::cmp::{min, Ordering};

struct Solution {}

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let len = words.len() as i32;

        let calculate_min = |closest, pos: i32| match pos.cmp(&start_index) {
            Ordering::Greater => min(
                min(closest, pos - start_index),
                min(closest, start_index + (len - pos)),
            ),
            _ => min(
                min(closest, start_index - pos),
                min(closest, pos + (len - start_index)),
            ),
        };

        let moves = words
            .iter()
            .enumerate()
            .filter(|(_, word)| &&target == word)
            .map(|(pos, _)| pos as i32)
            .fold(i32::MAX, |closest, pos| calculate_min(closest, pos));

        if moves == i32::MAX {
            return -1;
        }

        moves
    }
}

fn main() {
    let words = vec![
        "hello".to_owned(),
        "i".to_owned(),
        "am".to_owned(),
        "leetcode".to_owned(),
        "hello".to_owned(),
    ];
    let target = String::from("hello");
    println!("{}", Solution::closet_target(words, target, 1));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let words = vec!["a".to_owned(), "b".to_owned(), "leetcode".to_owned()];
        let target = String::from("leetcode");
        assert_eq!(Solution::closet_target(words, target, 0), 1);

        let words = vec!["i".to_owned(), "eat".to_owned(), "leetcode".to_owned()];
        let target = String::from("ate");
        assert_eq!(Solution::closet_target(words, target, 0), -1);
    }
}
