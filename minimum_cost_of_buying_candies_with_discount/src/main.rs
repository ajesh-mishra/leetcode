struct Solution {}

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_by(|a, b| b.cmp(a));
        let mut result = 0;
        for triplet in cost.chunks(3) {
            result += triplet.get(0).unwrap_or_else(|| &0) + triplet.get(1).unwrap_or_else(|| &0);
        }
        result
    }
}

fn main() {
    println!("{}", Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]));
}
