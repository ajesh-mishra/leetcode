struct Solution {}

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let total = arr.iter().sum::<i32>();
        if total % 3 != 0 {
            return false;
        }
        let sum_part = total / 3;
        let (mut n, mut acc) = (0, 0);
        for &val in &arr {
            acc += val;
            if acc == sum_part {
                n += 1;
                acc = 0;
            }
        }
        n >= 3
    }
}

fn main() {
    let arr = vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1];
    println!("{}", Solution::can_three_parts_equal_sum(arr));
}
