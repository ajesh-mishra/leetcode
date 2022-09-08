struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            if let Some(cs) = stack.last() {
                if *cs == c {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c)
        }
        stack.iter().collect()
    }
}

fn main() {
    println!("{}", Solution::remove_duplicates("abbaca".to_string()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::remove_duplicates("azxxzy".to_string()),
            "ay".to_string()
        );
    }
}
