use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            if !new_interval.is_empty() && interval[1] < new_interval[0] {
                result.push(interval.clone());
                continue;
            }
            if new_interval.is_empty() {
                result.push(interval);
                continue;
            }
            if interval[0] > new_interval[1] {
                result.push(new_interval.clone());
                new_interval.clear();
                result.push(interval);
                continue;
            }
            new_interval[0] = min(interval[0], new_interval[0]);
            new_interval[1] = max(interval[1], new_interval[1]);
        }
        if !new_interval.is_empty() {
            result.push(new_interval);
        }
        result
    }
}

fn main() {
    // let intervals = vec![vec![1, 3], vec![6, 9]];
    // let new_interval = vec![2, 5];
    // println!("{:?}", Solution::insert(intervals, new_interval));

    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    println!("{:?}", Solution::insert(intervals, new_interval));
}
