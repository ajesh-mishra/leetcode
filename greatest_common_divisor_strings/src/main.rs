struct Solution {}

impl Solution {
    fn calculate_gcd(mut x: usize, mut y: usize) -> usize {
        let mut rem = x % y;
        while rem != 0 {
            x = y;
            y = rem;
            rem = x % y;
        }
        y
    }
    fn calculate_str(pattern: &str, rep: usize) -> String {
        let mut str_calculated = String::from("");
        for _ in 0..rep {
            str_calculated.push_str(pattern);
        }
        str_calculated
    }
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let str1_len = str1.len();
        let str2_len = str2.len();

        let gcd = if str1_len < str2_len {
            Solution::calculate_gcd(str2_len, str1_len)
        } else {
            Solution::calculate_gcd(str1_len, str2_len)
        };

        let pattern = &str1[0..gcd];
        let str1_calculated = Solution::calculate_str(pattern, str1_len / gcd);
        let str2_calculated = Solution::calculate_str(pattern, str2_len / gcd);

        if str1_calculated == str1 && str2_calculated == str2 {
            return pattern.to_string();
        }

        String::from("")
    }
}

fn main() {
    let str1 = String::from("ABCABC");
    let str2 = String::from("ABC");
    println!("{}", Solution::gcd_of_strings(str1, str2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        assert_eq!(Solution::gcd_of_strings(str1, str2), String::from("AB"));
    }

    #[test]
    fn ut_false() {
        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        assert_eq!(Solution::gcd_of_strings(str1, str2), String::from(""));
    }
}