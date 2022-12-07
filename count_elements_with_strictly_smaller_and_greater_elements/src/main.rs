use std::slice::Iter;

struct Solution {}

impl Solution {
    fn count(n: &i32, nums: Iterator) -> i32 {
        let mut count = 0;
        for n in nums {
            if 
        }
    }
    pub fn count_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| a.cmp(b));
        let smallest = nums.first().unwrap();
        let largest = nums.last().unwrap();

        
        // for n in nums.iter() {
        //     if n == smallest || n == largest {
        //         count += 1;
        //     }
        // }

        // let x = nums.iter();
    
        nums.len() as i32 - Self::count(smallest, nums.iter())
    }
}

fn main() {
    let nums = vec![11, 7, 2, 15];
    println!("{}", Solution::count_elements(nums));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let nums = vec![-3, 3, 3, 90];
        assert_eq!(Solution::count_elements(nums), 2);
    }
}
