struct Solution {}

impl Solution {
    fn get_value(x: i32) -> (i32, i32) {
        let this_value: i32;
        let next_value: i32;

        if x % 2 == 0 {
            if let Some(y) = (x+1).checked_mul((x)/2) {
                this_value = y;
            } else {
                this_value = i32::MAX;
            }
            
            if let Some(y) = (x+1).checked_mul((x+2)/2) {
                next_value = y;
            } else {
                next_value = i32::MIN;
            }
        } else {
            if let Some(y) = (x).checked_mul((x+1)/2) {
                this_value = y;
            } else {
                this_value = i32::MAX;
            }
            
            if let Some(y) = (x+2).checked_mul((x+1)/2) {
                next_value = y;
            } else {
                next_value = i32::MIN;
            }
        }

        (this_value, next_value)
    }

    pub fn arrange_coins(n: i32) -> i32 {
        fn inner(start: i32, end: i32, n: i32) -> i32 {
            let x = start + (end - start) / 2;
            let (this_value, next_value) = Solution::get_value(x);

            // println!("{x}, {this_value}, {next_value}");

            if this_value <= n && next_value > n {
                return x;
            }
            
            if this_value > n {
                inner(start, x, n)
            } else {
                inner(x, end, n)
            }
        }

        inner(0, (n/2)+1, n)
    }
}

fn main() {
    println!("{}", Solution::arrange_coins(5));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert_eq!(Solution::arrange_coins(9), 3);
    }

    #[test]
    fn tc_2() {
        assert_eq!(Solution::arrange_coins(18), 5);
    }

    #[test]
    fn tc_3() {
        assert_eq!(Solution::arrange_coins(1_804_289_383), 60070);
    }
}
