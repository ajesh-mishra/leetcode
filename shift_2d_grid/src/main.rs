struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let row_len = grid[0].len();
        let flat_grid: Vec<i32> = grid.iter().flat_map(|x| x.clone()).collect();
        let flat_grid_len = flat_grid.len();
        let mut shifted_flat_grid: Vec<i32> = vec![0; flat_grid_len];

        let mut pos = if k as usize > flat_grid_len {
            k as usize % flat_grid_len
        } else {
            k as usize
        };

        for &i in &flat_grid {
            if pos == flat_grid_len {
                pos = 0;
            }
            shifted_flat_grid[pos] = i;
            pos += 1;
        }

        let mut result = Vec::new();
        for n in shifted_flat_grid.chunks_exact(row_len) {
            result.push(n.iter().map(|&x| x).collect::<Vec<i32>>());
        }

        result
    }
}

fn main() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", Solution::shift_grid(grid, 1));
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        assert_eq!(
            Solution::shift_grid(grid, 4),
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10]
            ]
        );

        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::shift_grid(grid, 9),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
        
        let grid = vec![ vec![1]];
        assert_eq!(
            Solution::shift_grid(grid, 100),
            vec![vec![1]]
        );
    }
}
