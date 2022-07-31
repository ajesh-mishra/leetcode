struct Solution {}

impl Solution {
    pub fn license_key_formatting_1(s: String, k: i32) -> String {
        let mut new_string = String::from("");

        for c in s.chars() {
            match c {
                'a'..='z' => {
                    new_string.push(c.to_ascii_uppercase());
                },
                '-' => {},
                _ => {
                    new_string.push(c);
                }
            }
        }

        if new_string.len() == 0 {
            return String::from("");
        }

        let first_seg_len = new_string.len() as i32 % k;
        let mut result = String::from("");
        let mut is_first = true;
        let mut count = 1;

        for c in new_string.chars() {
            result.push(c);
            if count == k || (count == first_seg_len && is_first) {
                result.push('-');
                count = 0;
                is_first = false;
            }
            count += 1;
        }

        result[0..result.len()-1].to_string()
    }

    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut new_string = String::from("");
        let mut count = 1;

        for c in s.chars().rev() {
            match c {
                'a'..='z' => {
                    // new_string.push(c.to_ascii_uppercase());
                    new_string = format!("{}{new_string}", c.to_ascii_uppercase());
                    if count == k {
                        new_string = format!("{}{new_string}", '-');
                        count = 0;
                    }
                    count += 1;
                },
                '-' => {},
                _ => {
                    new_string = format!("{}{new_string}", c);
                    if count == k {
                        new_string = format!("{}{new_string}", '-');
                        count = 0;
                    }
                    // new_string.push(c);
                    count += 1;
                }
            }
        }

        if let Some(x) = new_string.chars().next() {
            if x == '-' {
                new_string[1..new_string.len()].to_string()
            } else {
                new_string
            }
        } else {
            String::from("")
        }
    }
}

fn main() {
    let s = String::from("5F3Z-2e-9-w");
    let k = 4;
    println!("{}", Solution::license_key_formatting_1(s.clone(), k));
    println!("{}", Solution::license_key_formatting(s, k));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let s = String::from("2-5g-3-J");
        assert_eq!(Solution::license_key_formatting(s, 2), String::from("2-5G-3J"));
    }

    #[test]
    fn tc_2() {
        let s = String::from("---");
        assert_eq!(Solution::license_key_formatting(s, 3), String::from(""));
    }

    #[test]
    fn tc_3() {
        let s = String::from("5F3Z-2e-9-w");
        assert_eq!(Solution::license_key_formatting(s, 4), String::from("5F3Z-2E9W"));
    }
}
