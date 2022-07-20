pub struct Solution {}

impl Solution {
    // Division Method
    // pub fn is_power_of_two(mut n: i32) -> bool {
    //     if n == 0  { 
    //         return false;
    //     } 

    //     while n % 2 == 0 { 
    //         n /= 2;
    //     }

    //     n == 1
    // }

    // Bit Manipulation Method
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n-1) == 0
    }
}

fn main() {
    println!("{}", Solution::is_power_of_two(16));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert!(!Solution::is_power_of_two(1073741825));
    }

    #[test]
    fn tc_2() {
        assert!(!Solution::is_power_of_two(16777217));
    }

    #[test]
    fn tc_3() {
        assert!(Solution::is_power_of_two(512));
    }

    #[test]
    fn tc_4() {
        assert!(!Solution::is_power_of_two(1025));
    }

    #[test]
    fn tc_5() {
        assert!(Solution::is_power_of_two(1));
    }
}
