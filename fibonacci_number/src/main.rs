struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn inner(n: i32, curr: i32, mut a: [i32; 2]) -> i32 {
            let result = a[0] + a[1];
            if curr == n {
                return result;
            } else {
                a[0] = a[1];
                a[1] = result;
                inner(n, curr + 1, a)
            }
        }

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            inner(n, 2, [0, 1])
        }
    }
}

fn main() {
    println!("3 -> {}", Solution::fib(4));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(5), 5);
        assert_eq!(Solution::fib(8), 21);
    }
}
