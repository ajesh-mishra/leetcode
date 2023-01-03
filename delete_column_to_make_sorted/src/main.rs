struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let len = strs[0].len();
        let mut result: Vec<char> = Vec::with_capacity(len);
        for (i, word) in strs.iter().enumerate() {
            for (j, c) in word.char_indices() {
                if i == 0 {
                    result.push(c);
                    continue;
                }
                if result[j] != 'A' && result[j] <= c {
                    result[j] = c;
                } else {
                    result[j] = 'A';
                }
            }
        }
        result.iter().filter(|&&c| c == 'A').count() as _
    }
}

fn main() {
    let strs = vec!["cba".to_owned(), "daf".to_owned(), "ghi".to_owned()];
    println!("{}", Solution::min_deletion_size(strs));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let strs = vec!["zyx".to_owned(), "wvu".to_owned(), "tsr".to_owned()];
        assert_eq!(Solution::min_deletion_size(strs), 3);
        let strs = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(Solution::min_deletion_size(strs), 0);
    }

    #[test]
    fn ut_2() {
        let strs = vec!["rrjk".to_owned(), "furt".to_owned(), "guzm".to_owned()];
        assert_eq!(Solution::min_deletion_size(strs), 2);
    }
}
