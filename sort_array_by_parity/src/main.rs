struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_1(nums: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();

        for num in nums {
            match num % 2 {
                0 => even.push(num),
                _ => odd.push(num),
            }
        }

        even.extend(odd);
        even
    }

    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        for j in 0..nums.len() {
            println!("{i}, {j}");
            if nums[j] % 2 == 0 {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums
    }
}

fn main() {
    let nums = vec![3, 1, 2, 4];
    println!("{:?}", Solution::sort_array_by_parity_1(nums.clone()));
    println!("{:?}", Solution::sort_array_by_parity(nums));
}
