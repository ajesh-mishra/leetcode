struct Solution {}

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|&x| x >= 10).fold(0, |diff, num| {
            let (mut num2, mut digit_sum) = (num, 0);
            while num2 != 0 {
                let digit = num2 % 10;
                num2 /= 10;
                digit_sum += digit;
            }
            diff + (num - digit_sum)
        })
    }
    pub fn difference_of_sum_1(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        for num in nums {
            if num < 10 {
                continue;
            }
            let mut num2 = num;
            let mut digit_sum = 0;
            while num2 != 0 {
                let digit = num2 % 10;
                num2 /= 10;
                digit_sum += digit;
            }
            diff += num - digit_sum;
        }
        diff
    }
}

fn main() {
    let nums = vec![2, 7, 8, 10, 8, 10, 1, 10, 5, 9];
    println!("{}", Solution::difference_of_sum(nums));
    let nums = vec![2, 7, 8, 10, 8, 10, 1, 10, 5, 9];
    println!("{}", Solution::difference_of_sum_1(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        // let nums = vec![1, 15, 6, 3];
        // assert_eq!(Solution::difference_of_sum(nums), 9);
        // let nums = vec![1, 5, 6, 3];
        // assert_eq!(Solution::difference_of_sum(nums), 0);
        let nums = vec![2, 7, 8, 10, 8, 10, 1, 10, 5, 9];
        assert_eq!(Solution::difference_of_sum(nums), 27);
    }
}
