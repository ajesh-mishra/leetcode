struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut first = '_';
        let mut last = '_';
        for (i, word) in sentence.split(' ').enumerate() {
            let len = word.len();
            let f = word.chars().nth(0).unwrap();
            let l = word.chars().nth(len - 1).unwrap();
            if i == 0 {
                last = l;
                first = f;
                continue;
            }
            if last != f {
                return false;
            }
            last = l;
        }
        first == last
    }
}

fn main() {
    let sentence = String::from("leetcode exercises sound delightful");
    println!("{}", Solution::is_circular_sentence(sentence));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let sentence = String::from("leetcode exercises sound delightful");
        assert!(Solution::is_circular_sentence(sentence));
        let sentence = String::from("eetcode");
        assert!(Solution::is_circular_sentence(sentence));
    }

    #[test]
    fn ut_false() {
        let sentence = String::from("Leetcode is cool");
        assert!(!Solution::is_circular_sentence(sentence));
    }
}
