struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let l = words.len();
        words
            .into_iter()
            .flat_map(|s| s.into_bytes())
            .fold([0; 26], |mut counter, b| {
                counter[(b - b'a') as usize] += 1;
                counter
            })
            .iter()
            .all(|x| *x % l == 0)
    }
}

fn main() {
    let words = vec!["abc".to_owned(), "aabc".to_owned(), "bc".to_owned()];
    println!("{}", Solution::make_equal(words));
}
