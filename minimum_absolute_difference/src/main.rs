use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut diff = i32::MAX;
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        arr.sort_by(|a, b| a.cmp(b));

        for a in arr.windows(2) {
            let diff1 = a[1] - a[0];
            if diff1 <= diff {
                diff = diff1;
                let count = map.entry(diff).or_insert(vec![]);
                count.push(vec![a[0], a[1]]);
            }
        }

        let min = map.keys().min().unwrap();
        map.get(min).unwrap().clone()
    }
}

fn main() {
    let arr = vec![1, 3, 6, 10, 15];
    println!("{:?}", Solution::minimum_abs_difference(arr));
}
