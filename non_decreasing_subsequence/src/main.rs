use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn is_sorted(num: &[i32]) -> bool {
        let mut prev = i32::MIN;
        for &n in num {
            if prev > n {
                return false;
            }
            prev = n;
        }
        true
    }
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let nums2 = nums.clone();
        let len = nums.len();
        let mut result: HashSet<Vec<i32>> = HashSet::new();

        for w in 1..len + 1 {
            for (i, num) in nums.windows(w).enumerate() {
                // print!("{:?} ", num);
                if num.len() > 1 && Self::is_sorted(num) {
                    result.insert(num.iter().map(|&x| x).collect());
                }
                if w + i == len {
                    continue;
                }
                for j in w + 1..len {
                    print!(" {}", nums2[j]);
                    if num.last().unwrap() <= &nums2[j] {
                        let mut temp: Vec<i32> = num.iter().map(|&x| x).collect();
                        temp.push(nums2[j]);
                        result.insert(temp);
                    }
                }
            }
            println!("");
        }

        result.iter().map(|x| x.clone()).collect::<Vec<Vec<i32>>>()
        // println!("{:?}", result);
        // vec![]
    }
}

fn main() {
    let nums = vec![4, 6, 7, 7];
    println!("{:?}", Solution::find_subsequences(nums));
}
