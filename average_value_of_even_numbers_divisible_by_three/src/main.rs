struct Solution {}

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut running_total = 0;
        let mut count = 0;
        for n in nums {
            if n % 6 == 0 {
                running_total += n;
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        running_total / count
    }
}

fn main() {
    let nums = vec![1, 3, 6, 10, 12, 15];
    println!("{}", Solution::average_value(nums));
}
