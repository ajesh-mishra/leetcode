use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_map: HashMap<char, usize> = HashMap::new();

        for (i, c) in order.char_indices() {
            order_map.insert(c, i);
        }

        for word in words.windows(2) {
            let mut lengh_check = true;
            for (a, b) in word[0].chars().zip(word[1].chars()) {
                let av = order_map.get(&a).unwrap_or(&usize::MAX);
                let bv = order_map.get(&b).unwrap_or(&usize::MAX);
                if av > bv {
                    return false;
                } else if av < bv {
                    lengh_check = false;
                    break;
                }
            }
            if lengh_check && word[0].len() > word[1].len() {
                return false;
            }
        }

        true
    }
}

fn main() {
    let words = vec!["hello".to_string(), "leetcode".to_string()];
    let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
    println!("{}", Solution::is_alien_sorted(words, order));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = String::from("hlabcdefgijkmnopqrstuvwxyz");
        assert!(Solution::is_alien_sorted(words, order));

        let words = vec!["kuvp".to_string(), "q".to_string()];
        let order = String::from("ngxlkthsjuoqcpavbfdermiywz");
        assert!(Solution::is_alien_sorted(words, order));

        let words = vec!["apap".to_string(), "app".to_string()];
        let order = String::from("abcdefghijklmnopqrstuvwxyz");
        assert!(Solution::is_alien_sorted(words, order));
    }

    #[test]
    fn ut_false() {
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = String::from("worldabcefghijkmnpqstuvxyz");
        assert!(!Solution::is_alien_sorted(words, order));

        let words = vec!["apple".to_string(), "app".to_string()];
        let order = String::from("abcdefghijklmnopqrstuvwxyz");
        assert!(!Solution::is_alien_sorted(words, order));

        let words = vec![
            "fxasxpc".to_string(),
            "dfbdrifhp".to_string(),
            "nwzgs".to_string(),
            "cmwqriv".to_string(),
            "ebulyfyve".to_string(),
            "miracx".to_string(),
            "sxckdwzv".to_string(),
            "dtijzluhts".to_string(),
            "wwbmnge".to_string(),
            "qmjwymmyox".to_string(),
        ];
        let order = String::from("zkgwaverfimqxbnctdplsjyohu");
        assert!(!Solution::is_alien_sorted(words, order));
    }
}
