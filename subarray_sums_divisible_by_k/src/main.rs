struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut res, mut prefix) = (0, 0);
        let mut count = vec![0; (k + 1) as usize];
        count[0] += 1;

        for n in nums {
            prefix = (prefix + n).rem_euclid(k);
            println!("{}", res);
            res += count[prefix as usize];
            count[prefix as usize] += 1
        }

        println!("{:?}", count);

        res
    }
}

fn main() {
    let nums = vec![4, 5, 0, -2, -3, 1];
    println!("{}", Solution::subarrays_div_by_k(nums, 5));
}
