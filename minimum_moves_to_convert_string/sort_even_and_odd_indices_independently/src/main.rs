struct Solution {}

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        let mut even: Vec<i32> = nums.iter().step_by(2).map(|&x| x).collect();
        even.sort_by(|a, b| a.cmp(&b));
        let mut e = even.iter();
        // println!("{:?}", even);

        let mut odd: Vec<i32> = nums.iter().skip(1).step_by(2).map(|&x| x).collect();
        odd.sort_by(|a, b| b.cmp(&a));
        let mut o = odd.iter();
        // println!("{:?}", odd);

        for i in 0..nums.len() {
            match i % 2 {
                0 => result.push(*e.next().unwrap()),
                _ => result.push(*o.next().unwrap()),
            }
        }

        result
    }
}

fn main() {
    let nums = vec![4, 1, 2, 3];
    println!("{:?}", Solution::sort_even_odd(nums));
}
