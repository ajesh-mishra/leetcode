use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut incr = false;
        let mut decr = false;

        for duo in arr.windows(2) {
            match duo[1].cmp(&duo[0]) {
                Ordering::Greater if decr => return false,
                Ordering::Greater => incr = true,
                Ordering::Less => decr = true,
                Ordering::Equal => return false,
            }
        }

        incr && decr
    }
}

fn main() {
    let arr = vec![0, 3, 2, 1];
    println!("{}", Solution::valid_mountain_array(arr));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let arr = vec![0, 3, 2, 1];
        assert!(Solution::valid_mountain_array(arr));
    }

    #[test]
    fn ut_false() {
        let arr = vec![0, 3];
        assert!(!Solution::valid_mountain_array(arr));

        let arr = vec![3, 5, 5];
        assert!(!Solution::valid_mountain_array(arr));
    }
}
