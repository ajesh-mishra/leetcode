struct Solution {}

impl Solution {
    pub fn divisor_substrings(mut num: i32, k: i32) -> i32 {
        let (mut result, divident) = (0, num);
        let base = 10_i32.pow(k as u32 - 1);
        let factor = base * 10;

        while num >= base {
            let divisior = num % factor;
            if divisior != 0 && divident % divisior == 0 {
                result += 1;
            }
            num /= 10;
        }
        
        result
    }
}

fn main() {
    println!("{}", Solution::divisor_substrings(430043, 2));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
    }
}
