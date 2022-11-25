struct Solution {}

impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        nums.dedup();
        let mut count = 0;
        for win in nums.windows(3) {
            let (a, b, c) = (win[0], win[1], win[2]);
            match (b > a, b > c) {
                (true, true) => count += 1, 
                (false, false) => count += 1, 
                _ => {}
            }
        }
        count
    }
}

fn main() {
    let nums = vec![2,4,1,1,6,5];
    println!("{}", Solution::count_hill_valley(nums));
    let nums = vec![6,6,5,5,4,1];
    println!("{}", Solution::count_hill_valley(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![2,4,1,1,6,5];
        assert_eq!(Solution::count_hill_valley(nums), 3);
        let nums = vec![6,6,5,5,4,1];
        assert_eq!(Solution::count_hill_valley(nums), 0);
    }
}