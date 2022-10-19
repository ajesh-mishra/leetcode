use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        fn inner(num1: i32, num2: i32, counter: i32) -> i32 {
            match num1.cmp(&num2) {
                Ordering::Equal => counter,
                Ordering::Greater => inner(num1 - num2, num2, counter + 1),
                Ordering::Less => inner(num1, num2 - num1, counter + 1),
            }
        }
        if num1 == 0 || num2 == 0 {
            return 0;
        }
        inner(num1, num2, 1)
    }
}

fn main() {
    println!("{}", Solution::count_operations(2, 3));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::count_operations(10, 10), 1);
    }
}