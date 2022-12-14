use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid[0].len();
        let mut sorted_grid = vec![];
        let mut result = 0;
        for mut row in grid {
            row.sort_by(|a, b| b.cmp(a));
            sorted_grid.push(row);
        }
        for i in 0..len {
            let mut greatest = i32::MIN;
            for row in &sorted_grid {
                greatest = max(greatest, row[i]);
            }
            result += greatest;
        }
        result
    }
}

fn main() {
    let grid = vec![vec![1, 2, 4], vec![3, 3, 1]];
    println!("{}", Solution::delete_greatest_value(grid));
}
