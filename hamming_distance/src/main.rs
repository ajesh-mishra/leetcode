struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as _        
    }
}

fn main() {
    dbg!(Solution::hamming_distance(1, 4));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }

    #[test]
    fn tc_2() {
        assert_eq!(Solution::hamming_distance(93, 73), 2);
    }

    #[test]
    fn tc_3() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn tc_4() {
        assert_eq!(Solution::hamming_distance(1, 0), 1);
    }

    #[test]
    fn tc_5() {
        assert_eq!(Solution::hamming_distance(4, 14), 2);
    }
}
