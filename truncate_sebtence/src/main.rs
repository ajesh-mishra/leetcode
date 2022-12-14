struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result = String::from("");
        let mut count = 0;
        for c in s.chars() {
            match c {
                ' ' if count == k - 1 => break,
                ' ' => count += 1,
                _ => {}
            }
            result.push(c);
        }
        result
    }
}

fn main() {
    let s = String::from("Hello how are you Contestant");
    println!("{}", Solution::truncate_sentence(s, 4));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("hopper is not a tanuki");
        assert_eq!(
            Solution::truncate_sentence(s, 5),
            String::from("hopper is not a tanuki")
        );
    }
}
