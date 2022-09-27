struct Solution {}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        !n & ((n as u32).next_power_of_two() - 1).max(1) as i32         
    }
    pub fn bitwise_complement_1(n: i32) -> i32 {
        let nb = format!("{:b}", n);
        let mut cb = String::from("");
        let mut not_initial = false;
        for c in nb.chars() {
            match c {
                '0' => {
                    not_initial = true;
                    cb.push('1');
                }
                '1' if not_initial => cb.push('0'),
                '1' => {}
                _ => unreachable!(),
            }
        }
        let mut running = 1;
        let mut result = 0;
        for (i, c) in cb.chars().rev().enumerate() {
            if i != 0 {
                running *= 2;
            }
            if c == '1' {
                result += running;
            }
        }
        result
    }
}

fn main() {
    println!("{}", Solution::bitwise_complement(5));
    println!("{}", Solution::bitwise_complement_1(5));
}
