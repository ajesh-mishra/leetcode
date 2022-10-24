use std::collections::BinaryHeap;

#[derive(Debug)]
pub enum Parity {
    Even,
    Odd,
}

struct Solution {}

impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut sequence: Vec<Parity> = Vec::new();
        let mut even: BinaryHeap<i32> = BinaryHeap::new();
        let mut odd: BinaryHeap<i32> = BinaryHeap::new();
        while num != 0 {
            let digit: i32 = num % 10;
            match digit % 2 {
                0 => {
                    even.push(digit);
                    sequence.push(Parity::Even);
                }
                _ => {
                    odd.push(digit);
                    sequence.push(Parity::Odd);
                }
            }
            num /= 10;
        }
        let mut result: i32 = 0;
        for parity in sequence.iter().rev() {
            result = match parity {
                Parity::Even => (result * 10) + even.pop().unwrap(),
                Parity::Odd => (result * 10) + odd.pop().unwrap(),
            };
        }
        result
    }
}

fn main() {
    println!("{}", Solution::largest_integer(65875));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::largest_integer(65875), 87655);
    }
}