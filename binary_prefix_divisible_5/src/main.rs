struct Solution {}

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut running = 0;

        for num in nums {
            running = (running << 1 | num) % 5;
            result.push(running == 0);
        }

        result
    }
}

fn main() {
    let nums = vec![1, 1, 0];
    println!("{:?}", Solution::prefixes_div_by5(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::prefixes_div_by5(nums), vec![false, false, false]);
    }
}
