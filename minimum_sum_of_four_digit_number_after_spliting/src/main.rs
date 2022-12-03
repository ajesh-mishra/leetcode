use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut digits = BinaryHeap::new();
        while num != 0 {
            digits.push(num % 10);
            num = num / 10;
        }
        let mut first = 0;
        let mut second = 0;
        for i in 0..4 {
            if let Some(x) = digits.pop() {
                match i {
                    0 => first = x,
                    1 => second = x,
                    2 => first = (x * 10) + first,
                    3 => second = (x * 10) + second,
                    _ => {}
                }
            }
        }
        println!("{first}, {second}");
        first + second
    }
}

fn main() {
    println!("{}", Solution::minimum_sum(2932));
}
