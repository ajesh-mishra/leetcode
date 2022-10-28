struct Solution {}

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seqs: Vec<bool> = vec![false; 26];
        for c in s.chars() {
            let pos = c as usize - 97;
            match seqs[pos] {
                false => seqs[pos] = true,
                true => return c,
            }
        }
        unreachable!();
    }
}

fn main() {
    println!(
        "{}",
        Solution::repeated_character(String::from("abzccbaacz"))
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::repeated_character(String::from("abcdd")), 'd');
    }
}
