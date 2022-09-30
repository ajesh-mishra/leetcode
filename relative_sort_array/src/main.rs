use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count_map: HashMap<i32, u32> = HashMap::new();
        for i in arr1 {
            let count = count_map.entry(i).or_insert(0);
            *count += 1;
        }

        let mut result = Vec::new();
        for i in arr2 {
            let &count = count_map.get(&i).unwrap();
            count_map.remove(&i);
            for _ in 0..count {
                result.push(i);
            }
        }

        let mut rest = Vec::new();
        for (val, count) in count_map {
            for _ in 0..count {
                rest.push(val);
            }
        }
        
        rest.sort_by(|a, b| a.cmp(&b));
        result.extend(rest);
        result
    }
}

fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    println!("{:?}", Solution::relative_sort_array(arr1, arr2));
}
