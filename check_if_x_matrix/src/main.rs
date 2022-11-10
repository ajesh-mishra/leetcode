struct Solution {}

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let len = grid.len();
        for i in 0..len {
            for j in 0..len {
                if i == j || i + j == len - 1 {
                    if grid[i][j] == 0 {
                        return false;
                    }
                } else {
                    if grid[i][j] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    let grid = vec![
        vec![2, 0, 0, 1],
        vec![0, 3, 1, 0],
        vec![0, 5, 2, 0],
        vec![4, 0, 0, 2],
    ];
    println!("{}", Solution::check_x_matrix(grid));
}
