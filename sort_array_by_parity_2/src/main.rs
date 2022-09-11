struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii_1(mut nums: Vec<i32>) -> Vec<i32> {
        let mut evens = Vec::new();
        let mut odds = Vec::new();

        for i in 0..nums.len() {
            if i % 2 == 0 && nums[i] % 2 != 0 {
                if let Some(x) = evens.pop() {
                    let temp = nums[i];
                    nums[i] = nums[x];
                    nums[x] = temp;
                } else {
                    odds.push(i);
                }
            }
            if i % 2 != 0 && nums[i] % 2 == 0 {
                if let Some(x) = odds.pop() {
                    let temp = nums[i];
                    nums[i] = nums[x];
                    nums[x] = temp;
                } else {
                    evens.push(i);
                }
            }
        }

        nums
    }
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut evens = Vec::new();
        let mut odds = Vec::new();
        for num in &nums {
            if num % 2 == 0 {
                evens.push(*num);
            } else {
                odds.push(*num);
            }
        } 
        let mut count = 0;
        for (&x, &y) in (evens.iter()).zip(odds.iter()) {
            nums[count] = x;
            count += 1;
            nums[count] = y;
            count += 1;
        }
        nums
    }
}

fn main() {
    let nums = vec![4, 2, 5, 7];
    println!("{:?}", Solution::sort_array_by_parity_ii_1(nums));
    let nums = vec![4, 2, 5, 7];
    println!("{:?}", Solution::sort_array_by_parity_ii(nums));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let nums = vec![4, 2, 5, 7];
        assert_eq!(Solution::sort_array_by_parity_ii(nums), vec![4, 5, 2, 7]);
        let nums = vec![2, 3];
        assert_eq!(Solution::sort_array_by_parity_ii(nums), vec![2, 3]);
    }
}
