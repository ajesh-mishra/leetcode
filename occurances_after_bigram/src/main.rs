struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = Vec::new();
        for word in text.split(" ").collect::<Vec<&str>>().windows(3) {
            if word[0] == &first && word[1] == &second {
                result.push(word[2].to_string());
            }
        }
        result
    }
}

fn main() {
    let text = String::from("alice is a good girl she is a good student");
    let first = String::from("a");
    let second = String::from("good");
    println!("{:?}", Solution::find_ocurrences(text, first, second));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let text = String::from("we will we will rock you");
        let first = String::from("we");
        let second = String::from("will");
        assert_eq!(
            Solution::find_ocurrences(text, first, second),
            vec!["we".to_string(), "rock".to_string()]
        );
    }
}
