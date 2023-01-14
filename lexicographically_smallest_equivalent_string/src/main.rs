use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut reference: [Option<u8>; 26] = [None; 26];
        for (c1, c2) in s1.bytes().zip(s2.bytes()) {
            let mut group: HashSet<u8> = HashSet::from_iter([c1, c2]);
            let mut add_to_group = |c: u8| match reference[c as usize - 97] {
                None => reference[c as usize - 97] = Some(c),
                Some(v) => {
                    group.insert(v);
                }
            };
            add_to_group(c1);
            add_to_group(c2);
            let smallest = group.iter().min().unwrap();
            for i in 0..26 {
                if let Some(x) = reference[i] {
                    if group.contains(&x) {
                        reference[i] = Some(*smallest);
                    }
                }
            }
        }
        String::from_utf8(
            base_str
                .bytes()
                .map(|x| match reference[x as usize - 97] {
                    Some(p) => p,
                    None => x,
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

fn main() {
    let s1 = String::from("parker");
    let s2 = String::from("morris");
    let base_str = String::from("parser");
    println!(
        "{} -> {}",
        Solution::smallest_equivalent_string(s1, s2, base_str),
        String::from("makkek")
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s1 = String::from("parker");
        let s2 = String::from("morris");
        let base_str = String::from("parser");
        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            String::from("makkek")
        );

        let s1 = String::from("hello");
        let s2 = String::from("world");
        let base_str = String::from("hold");
        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            String::from("hdld")
        );

        let s1 = String::from("leetcode");
        let s2 = String::from("programs");
        let base_str = String::from("sourcecode");
        assert_eq!(
            Solution::smallest_equivalent_string(s1, s2, base_str),
            String::from("aauaaaaada")
        );
    }
}
