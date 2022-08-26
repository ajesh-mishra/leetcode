struct Solution {}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for number in left..=right {
            let mut is_self_dividing = true;

            for c in format!("{number}").chars() {
                let digit = c.to_digit(10).unwrap();
                if digit == 0 || number % digit as i32 != 0 {
                    is_self_dividing = false;
                    break;
                }
            }

            if is_self_dividing {
                result.push(number);
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::self_dividing_numbers(47, 85));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }
}
