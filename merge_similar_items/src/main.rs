use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn merge_similar_items(
        mut items1: Vec<Vec<i32>>,
        mut items2: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        items1.append(&mut items2);
        let mut map = HashMap::new();
        for item in items1 {
            let k = item[0];
            let w = item[1];
            let weight = map.entry(k).or_insert(0);
            *weight += w;
        }
        let mut result: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        result.sort_by(|(a, _), (b, _)| a.cmp(b));
        result.iter().map(|(a, b)| vec![*a, *b]).collect()
    }
}

fn main() {
    let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let items2 = vec![vec![3, 1], vec![1, 5]];
    println!("{:?}", Solution::merge_similar_items(items1, items2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let items1 = vec![vec![1, 3], vec![2, 2]];
        let items2 = vec![vec![7, 1], vec![2, 2], vec![1, 4]];
        assert_eq!(
            Solution::merge_similar_items(items1, items2),
            vec![vec![1, 7], vec![2, 4], vec![7, 1]]
        );

        let items1 = vec![vec![1, 1], vec![3, 2], vec![2, 3]];
        let items2 = vec![vec![2, 1], vec![3, 2], vec![1, 3]];
        assert_eq!(
            Solution::merge_similar_items(items1, items2),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
    }
}
