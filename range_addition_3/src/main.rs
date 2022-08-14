struct Solution {}

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.is_empty() {
            return m * n;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;

        for op in ops {
            if op[1] < min_y {
                min_y = op[1];
            }
            if op[0] < min_x {
                min_x = op[0];
            }
        }

        min_x * min_y
    }
}

fn main() {
    let ops = vec![vec![2, 2], vec![3, 3]];
    println!("{}", Solution::max_count(3, 3, ops));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_positive_case() {
        let ops = vec![
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
        ];
        assert_eq!(Solution::max_count(3, 3, ops), 4);

        let ops = vec![
            vec![10, 31],
            vec![1, 4],
            vec![9, 24],
            vec![13, 24],
            vec![13, 25],
            vec![8, 26],
            vec![3, 26],
            vec![5, 25],
            vec![10, 32],
            vec![5, 15],
            vec![10, 24],
            vec![9, 7],
        ];
        assert_eq!(Solution::max_count(14, 38, ops), 4);
    }

    #[test]
    fn ut_empty_ops() {
        let ops = vec![];
        assert_eq!(Solution::max_count(3, 3, ops), 9);

        let ops = vec![];
        assert_eq!(Solution::max_count(40_000, 40_000, ops), 1_600_000_000);
    }
}
