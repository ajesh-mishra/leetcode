use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = grid.len();
        let local_len = len - 2;
        let mut possible_pos = vec![];
        let mut result = vec![vec![0; local_len]; local_len];

        for i in 0..local_len {
            for j in 0..local_len {
                possible_pos.push((i, j));
            }
        }

        for (i, j) in possible_pos {
            let mut max_value = i32::MIN;
            for i_local in i..i + 3 {
                for j_local in j..j + 3 {
                    max_value = max(grid[i_local][j_local], max_value);
                }
            }
            result[i][j] = max_value;
        }

        result
    }
}

fn main() {
    let grid = vec![
        vec![9, 9, 8, 1],
        vec![5, 6, 2, 6],
        vec![8, 2, 6, 4],
        vec![6, 2, 2, 2],
    ];
    println!("{:?}", Solution::largest_local(grid));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(
            Solution::largest_local(grid),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
