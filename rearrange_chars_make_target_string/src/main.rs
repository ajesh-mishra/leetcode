use std::{cmp::min, collections::HashMap};

struct Solution {}

impl Solution {
    fn get_count(s: &str) -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        map
    }
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let map_s = Self::get_count(s.as_str());
        let map_t = Self::get_count(target.as_str());
        let mut result = i32::MAX;
        for (c, t_count) in map_t {
            if let Some(&s_count) = map_s.get(&c) {
                result = min(result, s_count / t_count);
            } else {
                return 0;
            }
        }
        result
    }
}

fn main() {
    let s = String::from("ilovecodingonleetcode");
    let target = String::from("code");
    println!("{}", Solution::rearrange_characters(s, target));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("abcba");
        let target = String::from("abc");
        assert_eq!(Solution::rearrange_characters(s, target), 1);

        let s = String::from("abbaccaddaeea");
        let target = String::from("aaaaa");
        assert_eq!(Solution::rearrange_characters(s, target), 1);
    }
}
