use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut costs = Vec::new();
        let unique: HashSet<i32> = HashSet::from_iter(position.clone());

        for i in unique {
            let mut cost = 0;
            for p in position.iter() {
                if (p - i).abs() % 2 == 1 {
                    cost += 1;
                }
            }
            costs.push(cost);
        }

        *costs.iter().min().unwrap()
    }
}

fn main() {
    let position = vec![2,2,2,3,3];
    println!("{}", Solution::min_cost_to_move_chips(position));
}
