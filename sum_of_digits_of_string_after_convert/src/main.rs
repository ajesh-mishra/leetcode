struct Solution {}

impl Solution {
    pub fn get_lucky_1(s: String, k: i32) -> i32 {
        let mut s_bytes = s
            .bytes()
            .fold(String::from(""), |b, c| format!("{}{}", b, c - 96));
        let mut result = 0;
        for _ in 0..k {
            result = s_bytes.chars().fold(0, |r, c| r + c.to_digit(10).unwrap());
            s_bytes = format!("{}", result);
        }
        result as _
    }
    pub fn get_lucky_2(s: String, k: i32) -> i32 {
        let mut b = String::from("");
        for c in s.bytes() {
            b = format!("{}{}", b, c - 96);
        }
        let mut result = 0;
        for _ in 0..k {
            result = 0;
            for c in b.chars() {
                result += c.to_digit(10).unwrap();
            }
            b = format!("{}", result);
        }
        result as i32
    }
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut result = 0;
        let mut final_result = 0;
        let add_digits = |f: &mut i32, mut r: i32| {
            while r != 0 {
                *f += r % 10;
                r /= 10;
            }
        };
        for c in s.bytes() {
            let c = (c - 96) as i32;
            add_digits(&mut result, c);
        }
        if k == 1 {
            return result;
        }
        for _ in 1..k {
            final_result = 0;
            add_digits(&mut final_result, result);
            result = final_result;
        }
        final_result
    }
}

fn main() {
    let s = String::from("iiii");
    println!("{}", Solution::get_lucky(s, 1));
    let s = String::from("leetcode");
    println!("{}", Solution::get_lucky_2(s, 2));
    let s = String::from("qhquvppzooyt");
    println!("{}", Solution::get_lucky_1(s, 6));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("iiii");
        assert_eq!(Solution::get_lucky(s, 1), 36);
        let s = String::from("leetcode");
        assert_eq!(Solution::get_lucky(s, 2), 6);
        let s = String::from("qhquvppzooyt");
        assert_eq!(Solution::get_lucky(s, 6), 2);
    }
}
