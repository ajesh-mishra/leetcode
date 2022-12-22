struct Solution {}

impl Solution {
    pub fn check_if_pangram_1(sentence: String) -> bool {
        let mut check = [false; 26];
        for c in sentence.bytes() {
            let pos = c - 97;
            check[pos as usize] = true;
        }
        check.iter().all(|&x| x)
    }
}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence
            .as_bytes()
            .iter()
            .fold([false; 26], |mut count_arr, &x| {
                count_arr[(x - b'a') as usize] = true;
                count_arr
            })
            .iter()
            .all(|&x| x)
    }
}

fn main() {
    let sentence = String::from("thequickbrownfoxjumpsoverthelazydog");
    println!("{}", Solution::check_if_pangram(sentence));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_true() {
        let sentence = String::from("thequickbrownfoxjumpsoverthelazydog");
        assert!(Solution::check_if_pangram(sentence));
    }

    #[test]
    fn ut_false() {
        let sentence = String::from("leetcode");
        assert!(!Solution::check_if_pangram(sentence));
    }
}
