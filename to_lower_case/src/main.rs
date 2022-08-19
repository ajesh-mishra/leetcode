struct Solution {}

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let result = s.bytes()
            .map(|c| {
                match c {
                    b'A'..=b'Z' => c + (b'a' - b'A'),
                    _ => c,
                }
            })
            .collect();

        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let s = String::from("Hello!");
    println!("{}", Solution::to_lower_case(s));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_chars() {
        let s = String::from("Hello!");
        assert_eq!(Solution::to_lower_case(s), String::from("hello!"));
        let s = String::from("Hello World !");
        assert_eq!(Solution::to_lower_case(s), String::from("hello world !"));
    }

    #[test]
    fn ut_ascii() {
        let s = String::from("Hello 😎!");
        assert_eq!(Solution::to_lower_case(s), String::from("hello 😎!"));
    }
}