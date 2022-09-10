use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {}

impl Solution {
    fn find_uncommon_words(
        sentence: String,
        mut repeated: HashSet<String>,
    ) -> (HashSet<String>, HashSet<String>) {
        let mut v: Vec<String> = Vec::new();
        let mut temp: HashSet<String> = HashSet::new();

        for st in sentence.split(" ") {
            let s = st.to_string();
            if temp.insert(s.clone()) {
                v.push(s);
            } else {
                repeated.insert(s.clone());
                v.retain(|x| x != &s);
            }
        }
        (HashSet::from_iter(v), repeated)
    }

    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let repeated: HashSet<String> = HashSet::new();
        let (v1, repeated) = Solution::find_uncommon_words(s1, repeated);
        let (v2, repeated) = Solution::find_uncommon_words(s2, repeated);

        v1.symmetric_difference(&v2)
            .filter(|s| !repeated.contains(*s))
            .map(|x| x.to_string())
            .collect()
    }
}

fn main() {
    let s1 = "this apple is sweet".to_string();
    let s2 = "this apple is sour".to_string();
    println!("{:?}", Solution::uncommon_from_sentences(s1, s2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s1 = "apple apple".to_string();
        let s2 = "banana".to_string();
        assert_eq!(
            Solution::uncommon_from_sentences(s1, s2),
            vec!["banana".to_string()]
        );
    }

    #[test]
    fn ut_2() {
        let s1 = "s z z z s".to_string();
        let s2 = "s z ejt".to_string();
        assert_eq!(
            Solution::uncommon_from_sentences(s1, s2),
            vec!["ejt".to_string()]
        );
    }
}
