struct Solution {}

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let (mut count, mut n) = (0, num);
        while n != 0 {
            let digit = n % 10;
            if digit == 0 {
                continue;
            }
            if num % digit == 0 {
                count += 1;
            }
            n /= 10;
        }
        count
    }
}

fn main() {
    println!("{}", Solution::count_digits(121));
}
