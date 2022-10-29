use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Position {
    height: i32,
    name: String,
}

struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut sorted_heights: BinaryHeap<Position> = BinaryHeap::new();
        for (&h, n) in heights.iter().zip(names.iter()) {
            sorted_heights.push(Position {
                height: h,
                name: n.to_owned(),
            });
        }
        let mut result = vec![];
        while let Some(x) = sorted_heights.pop() {
            result.push(x.name);
        }
        result
    }
}

fn main() {
    let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
    let heights = vec![180, 165, 170];
    println!("{:?}", Solution::sort_people(names, heights));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155, 185, 150];
        assert_eq!(
            Solution::sort_people(names, heights),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
