struct Solution {}

impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut is_first = true;
        let mut result = 0;
        for c in format!("{num}").chars() {
            result = match c {
                '6' if is_first => {
                    is_first = false;
                    (result * 10) + 9
                },
                '6' => (result * 10) + 6,
                _ => (result * 10) + 9,
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::maximum69_number(99669));
}
