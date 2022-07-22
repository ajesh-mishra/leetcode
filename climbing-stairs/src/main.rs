struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn fib_memo(n: usize, memo: &mut [usize; 2]) -> i32 {
            let [a, b] = *memo;
            let c = a + b;
            if n == 0 {
                c as i32
            } else {
                *memo = [b, c];
                fib_memo(n as usize - 1, memo) as i32
            }
        }

        if n < 2 {
            1
        } else {
            fib_memo(n as usize - 2, &mut [1, 1]) as i32
        }
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(2));
}
