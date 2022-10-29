struct Solution {}

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count = 0;
        for c in s.chars() {
            if c == letter {
                count += 1;
            }
        }
        (count * 100) / s.len() as i32
    }
}

fn main() {
    let s = String::from("foobar");
    let letter = 'o';
    println!("{}", Solution::percentage_letter(s, letter));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("jjjj");
        let letter = 'k';
        assert_eq!(Solution::percentage_letter(s, letter), 0);
    }
}
