struct Solution {}

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut result = [false; 10];
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    let x = char::to_digit(c, 10).unwrap();
                    if let Some(x) = result.get_mut(x as usize) {
                        *x = true;
                    }
                }
                _ => {}
            }
        }
        let mut is_first = false;
        for (i, is_true) in result.iter().rev().enumerate() {
            match (*is_true, is_first) {
                (true, true) => return 9 - i as i32,
                (true, _) => is_first = true,
                (false, _) => {}
            }
        }
        -1
    }
}

fn main() {
    let s = String::from("dfa12321afd");
    println!("{}", Solution::second_highest(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("abc1111");
        assert_eq!(Solution::second_highest(s), -1);
    }
}
