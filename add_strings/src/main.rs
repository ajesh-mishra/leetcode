struct Solution {}

impl Solution {
    pub fn add_strings_1(num1: String, num2: String) -> String {
        let mut res = Vec::with_capacity(num1.len().max(num2.len()));
        let mut num1 = num1.chars().rev();
        let mut num2 = num2.chars().rev();
        let mut carry = 0;
        let to_digit = |c: char| c as u8 - b'0';

        loop {
            let sum = carry
                + match (num1.next(), num2.next()) {
                    (Some(d1), Some(d2)) => to_digit(d1) + to_digit(d2),
                    (Some(d1), None) => to_digit(d1),
                    (None, Some(d2)) => to_digit(d2),
                    (None, None) => {
                        if carry > 0 {
                            res.push((carry + b'0') as char);
                        }
                        break;
                    }
                };
            res.push((sum % 10 + b'0') as char);
            carry = sum / 10;
        }
        res.into_iter().rev().collect()
    }

    pub fn add_strings(num1: String, num2: String) -> String {
        let mut n1 = num1.chars();
        let mut n2 = num2.chars();
        let mut result = String::from("");
        let mut carry: u32 = 0;
        let mut counter = if num1.len() > num2.len() {
            num1.len()
        } else {
            num2.len()
        };

        loop {
            let mut n1d = 0;
            let mut n2d = 0;

            if let Some(n) = n1.next_back() {
                if let Some(nd) = n.to_digit(10) {
                    n1d = nd;
                }
            } else {
                n1d = 0;
            }

            if let Some(n) = n2.next_back() {
                if let Some(nd) = n.to_digit(10) {
                    n2d = nd;
                }
            } else {
                n2d = 0;
            }

            if counter == 0 {
                if carry != 0 {
                    result = format!("{}{result}", 1);
                }
                break;
            }

            let x = n1d + n2d + carry;

            if x >= 10 {
                carry = 1;
                result = format!("{}{result}", x % 10);
            } else {
                carry = 0;
                result = format!("{x}{result}");
            }

            counter -= 1;
        }

        result
    }
}

fn main() {
    let num1 = String::from("11");
    let num2 = String::from("123");
    println!(
        "134 - {}",
        Solution::add_strings_1(num1.clone(), num2.clone())
    );
    println!("134 - {}", Solution::add_strings(num1, num2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let num1 = String::from("11");
        let num2 = String::from("123");
        assert_eq!(Solution::add_strings(num1, num2), String::from("134"));
    }

    #[test]
    fn tc_2() {
        let num1 = String::from("101");
        let num2 = String::from("123");
        assert_eq!(Solution::add_strings(num1, num2), String::from("224"));
    }

    #[test]
    fn tc_3() {
        let num1 = String::from("100");
        let num2 = String::from("123");
        assert_eq!(Solution::add_strings(num1, num2), String::from("223"));
    }

    #[test]
    fn tc_4() {
        let num1 = String::from("167141802233061013023557397451289113296441069");
        let num2 = String::from("401716832807512840963");
        assert_eq!(
            Solution::add_strings(num1, num2),
            String::from("167141802233061013023557799168121920809282032")
        );
    }

    #[test]
    fn tc_5() {
        let num1 = String::from("99");
        let num2 = String::from("99");
        assert_eq!(Solution::add_strings(num1, num2), String::from("198"));
    }
}
