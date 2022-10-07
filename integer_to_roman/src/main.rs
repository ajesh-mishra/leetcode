struct Solution {}

impl Solution {
    fn convert(num: i32, mut s: String) -> String {
        match num {
            0 => return s,
            4 => return String::from("IV"),
            9 => return String::from("IX"),
            40 => return String::from("XL"),
            90 => return String::from("XC"),
            400 => return String::from("CD"),
            900 => return String::from("CM"),
            _ => {}
        }

        let conversion = vec![
            (1000, 'M'),
            (500, 'D'),
            (100, 'C'),
            (50, 'L'),
            (10, 'X'),
            (5, 'V'),
            (1, 'I'),
        ];

        for (n, c) in conversion {
            if num == n {
                s.push(c);
                return s;
            } else if num > n {
                for _ in 0..num / n {
                    s.push(c);
                }
                return Self::convert(num % n, s.clone());
            }
        };

        unreachable!()
    }
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::from("");
        let mut factor = 1;
        while num != 0 {
            let digit = num % 10;
            result = format!("{}{result}", Self::convert(digit * factor, String::from("")));
            num = num / 10;
            factor *= 10;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(49));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_adding() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
        assert_eq!(Solution::int_to_roman(77), String::from("LXXVII"));
        assert_eq!(Solution::int_to_roman(1088), String::from("MLXXXVIII"));
    }

    #[test]
    fn ut_subtracting() {
        assert_eq!(Solution::int_to_roman(49), String::from("XLIX"));
        assert_eq!(Solution::int_to_roman(94), String::from("XCIV"));
        assert_eq!(Solution::int_to_roman(1494), String::from("MCDXCIV"));
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}