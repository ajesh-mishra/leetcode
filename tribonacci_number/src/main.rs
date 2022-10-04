struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        fn inner(n: i32, mut tribo: [i32; 3]) -> i32 {
            let tribo_sum = tribo.iter().sum();
            if n == 3 {
                return tribo_sum;
            }
            tribo = [tribo[1], tribo[2], tribo_sum];
            inner(n - 1, tribo)
        }
        match n {
            0 => return 0,
            1 | 2 => return 1,
            _ => inner(n, [0, 1, 1]),
        }
    }
}

fn main() {
    println!("{}", Solution::tribonacci(4));
    println!("1389537: {}", Solution::tribonacci(25));
}
