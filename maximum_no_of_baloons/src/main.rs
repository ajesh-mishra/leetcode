use std::collections::HashMap;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // balloon
        let mut l_count = 0;
        let mut o_count = 0;
        let initial = [('b', 0), ('a', 0), ('n', 0), ('l', 0), ('o', 0)];
        let mut count_map: HashMap<char, i32> = HashMap::from_iter(initial);

        let mut increment = |c: char| {
            let count = count_map.entry(c).or_insert(0);
            *count += 1;
        };

        for c in text.chars() {
            match c {
                'b' | 'a' | 'n' => increment(c),
                'l' => l_count += 1,
                'o' => o_count += 1,
                _ => {}
            }
            if l_count == 2 {
                l_count = 0;
                increment(c);
            }
            if o_count == 2 {
                o_count = 0;
                increment(c);
            }
        }
        // println!("{:#?}", count_map);
        *count_map.values().min().unwrap_or(&0)
    }
}

fn main() {
    let text = String::from("nlaebolko");
    println!("{}", Solution::max_number_of_balloons(text));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let text = String::from("loonbalxballpoon");
        assert_eq!(Solution::max_number_of_balloons(text), 2);
    }

    #[test]
    fn ut_nonw() {
        let text = String::from("leetcode");
        assert_eq!(Solution::max_number_of_balloons(text), 0);
        let text = String::from("lloo");
        assert_eq!(Solution::max_number_of_balloons(text), 0);
    }
}
