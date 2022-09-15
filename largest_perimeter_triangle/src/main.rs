struct Solution {}

impl Solution {
    pub fn largest_perimeter_my(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    match nums[i] < nums[j] + nums[k] {
                        true => return nums[i] + nums[j] + nums[k],
                        false => {}
                    }
                }
            }
        }

        0
    }

    pub fn largest_perimeter_fast(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        match nums.windows(3).find(|l| l[0] < l[1] + l[2]) {
            Some(l) => l.iter().sum(),
            None => 0,
        }
    }

    pub fn largest_perimeter_faster(nums: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        let (mut b, mut c) = (-1, -1);
        while let Some(a) = heap.pop() {
            if a > 0 && b > 0 && c > 0 && c < b + a {
                return a + b + c;
            }
            c = b;
            b = a;
        }
        0
    }
}

fn main() {
    let heights = vec![2, 1, 2];
    println!("{}", Solution::largest_perimeter_my(heights));

    let heights = vec![2, 1, 2];
    println!("{}", Solution::largest_perimeter_fast(heights));

    let heights = vec![1, 2, 1];
    println!("{}", Solution::largest_perimeter_faster(heights));
}
