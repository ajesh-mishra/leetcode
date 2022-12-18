struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut factor: i32 = 1;
        let mut result: i32 = 0;
        let x_str = match x < 0 {
            true => format!("{}", x * -1),
            false => format!("{}", x),
        };
        for (i, c) in x_str.char_indices() {
            if let Some(x) = c.to_digit(10) {
                match (i, factor.checked_mul(10)) {
                    (0, _) => {}
                    (_, Some(x)) => factor = x,
                    (_, None) => return 0,
                }
                match factor.checked_mul(x as i32) {
                    Some(x) => match result.checked_add(x) {
                        Some(x) => result = x,
                        None => return 0,
                    },
                    None => return 0,
                }
            }
        }
        match x < 0 {
            true => result * -1,
            false => result,
        }
    }
}

fn main() {
    println!("{}", Solution::reverse(123));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn ut_2() {
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(1563847412), 0);
    }
}
