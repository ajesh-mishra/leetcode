struct Solution {}

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        if s.len() <= k as usize {
            return s;
        }

        let mut arr = Vec::new();
        let mut temp: Option<u32> = None;

        for (i, c) in s.char_indices() {
            temp = match temp {
                Some(x) => Some(x + c.to_digit(10).unwrap()),
                None => c.to_digit(10),
            };
            match (i + 1) % k as usize {
                0 => {
                    if let Some(x) = temp {
                        arr.push(x);
                    }
                    temp = None;
                }
                _ => {}
            }
        }

        if let Some(x) = temp {
            arr.push(x);
        }

        let arr_string = arr.into_iter().map(|i| i.to_string()).collect::<String>();
        if arr_string.len() > k as usize {
            return Self::digit_sum(arr_string.clone(), k);
        }

        arr_string
    }
}

fn main() {
    let s = String::from("11111222223");
    println!("{}", Solution::digit_sum(s, 3));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let s = String::from("11111222223");
        assert_eq!(Solution::digit_sum(s, 3), String::from("135"));
        let s = String::from("00000000");
        assert_eq!(Solution::digit_sum(s, 3), String::from("000"));
        let s = String::from("01234567890");
        assert_eq!(Solution::digit_sum(s, 2), String::from("27"));
    }
}