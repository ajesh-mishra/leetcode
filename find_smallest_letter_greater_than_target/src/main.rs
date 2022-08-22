struct Solution {}

impl Solution {
    // Using Brut Force
    pub fn next_greatest_letter_1(letters: Vec<char>, target: char) -> char {
        for &letter in &letters {
            if letter > target {
                return letter;
            }
        }
        letters[0]
    }

    // Using Recursion 
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let len = letters.len();

        fn inner(start: usize, end: usize, letters: Vec<char>, target: char) -> char {
            let mid = (start + end) / 2;

            if letters[mid - 1] <= target && letters[mid] > target {
                return letters[mid];
            } else if letters[mid] > target {
                inner(start, mid, letters, target)
            } else {
                inner(mid, end, letters, target)
            }
        }

        if target < letters[0] || target >= letters[len - 1] {
            return letters[0];
        }

        inner(0, len, letters, target)
    }
}

fn main() {
    let letters = vec!['c', 'f', 'j'];
    println!("{}", Solution::next_greatest_letter_1(letters, 'a'));

    let letters = vec!['c', 'f', 'j'];
    println!("{}", Solution::next_greatest_letter(letters, 'a'));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_normal() {
        let letters = vec!['c', 'f', 'j'];
        assert_eq!(Solution::next_greatest_letter(letters, 'a'), 'c');

        let letters = vec!['c', 'f', 'j'];
        assert_eq!(Solution::next_greatest_letter(letters, 'c'), 'f');

        let letters = vec!['c', 'f', 'j'];
        assert_eq!(Solution::next_greatest_letter(letters, 'd'), 'f');
    }

    #[test]
    fn ut_repeated() {
        let letters = vec!['e', 'e', 'e', 'e', 'e', 'e', 'n', 'n', 'n', 'n'];
        assert_eq!(Solution::next_greatest_letter(letters, 'e'), 'n');

        let letters = vec!['e', 'e', 'e', 'k', 'q', 'q', 'q', 'v', 'v', 'y'];
        assert_eq!(Solution::next_greatest_letter(letters, 'q'), 'v');
    }

    #[test]
    fn ut_wrap() {
        let letters = vec!['a', 'b'];
        assert_eq!(Solution::next_greatest_letter(letters, 'z'), 'a');

        let letters = vec!['c', 'f', 'j'];
        assert_eq!(Solution::next_greatest_letter(letters, 'j'), 'c');
    }
}
