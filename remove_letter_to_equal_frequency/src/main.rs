use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut map = HashMap::new();
        for c in word.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
        if map.len() == 1 && map.values().all(|&x| x > 1) {
            return true;
        }
        if map.len() == 2 && map.values().any(|&x| x == 1) {
            return true;
        }
        if map.values().all(|&x| x == 1) {
            return true;
        }
        let mut frequency = HashMap::new();
        for (_, count) in map {
            let f = frequency.entry(count).or_insert(0);
            *f += 1;
        }
        println!("{:?}", frequency);
        // if frequency.len() == 3 && frequency.
        if frequency.len() != 2 {
            return false;
        }
        let keys: Vec<i32> = frequency.keys().map(|&x| x).collect();
        if keys[0] - keys[1] == 1 && *frequency.get(&keys[0]).unwrap() == 1 {
            return true;
        }
        if keys[1] - keys[0] == 1 && *frequency.get(&keys[1]).unwrap() == 1 {
            return true;
        }
        if let Some(&x) = frequency.get(&1) {
            // println!("inside the last true check");
            if x == 1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let word = String::from("abc");
    println!("{}", Solution::equal_frequency(word));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let word = String::from("abbcc");
        assert!(Solution::equal_frequency(word));
        let word = String::from("abcc");
        assert!(Solution::equal_frequency(word));
        let word = String::from("accc");
        assert!(Solution::equal_frequency(word));
        let word = String::from("bac");
        assert!(Solution::equal_frequency(word));
        let word = String::from("zz");
        assert!(Solution::equal_frequency(word));
    }

    #[test]
    fn ut_false() {
        let word = String::from("aazz");
        assert!(!Solution::equal_frequency(word));
        let word = String::from("aaaabbbbccc");
        assert!(!Solution::equal_frequency(word));
    }
}