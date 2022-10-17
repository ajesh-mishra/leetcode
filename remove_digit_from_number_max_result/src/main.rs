struct Solution {}

impl Solution {
    pub fn remove_digit(mut number: String, digit: char) -> String {
        let mut pos = 0;
        number.push('a');
        for (i, (a, b)) in number.chars().zip(number.chars().skip(1)).enumerate() {
            if a == digit {
                pos = i;
                if a < b {
                    pos = i;
                    break;
                }
            }
        }
        number.pop();
        number.remove(pos);
        number
    }
}

fn main() {
    let number = String::from("1231");
    let digit = '1';
    println!("{}", Solution::remove_digit(number, digit));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let number = String::from("1031");
        let digit = '1';
        assert_eq!(Solution::remove_digit(number, digit), String::from("103"));

        let number = String::from("1231");
        let digit = '1';
        assert_eq!(Solution::remove_digit(number, digit), String::from("231"));

        let number = String::from("2464");
        let digit = '6';
        assert_eq!(Solution::remove_digit(number, digit), String::from("244"));
    }
}
