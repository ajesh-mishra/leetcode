struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let matrix_len = matrix.len();
        let mut draft = Vec::new();
        for row in &matrix {
            let (mut pos, mut smallest) = (0, i32::MAX);
            for (i, &n) in row.iter().enumerate() {
                if n < smallest {
                    smallest = n;
                    pos = i;
                }
            }
            draft.push((smallest, pos));
        }
        let mut result = Vec::new();
        for (num, pos) in draft {
            let mut to_add = true;
            for i in 0..matrix_len {
                if num < matrix[i][pos] {
                    to_add = false;
                    break;
                }
            }
            if to_add {
                result.push(num);
            }
        }
        result
    }
}

fn main() {
    let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
    println!("{:?}", Solution::lucky_numbers(matrix));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let matrix = vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]];
        assert_eq!(Solution::lucky_numbers(matrix), vec![12]);
        let matrix = vec![vec![7, 8], vec![1, 2]];
        assert_eq!(Solution::lucky_numbers(matrix), vec![7]);
    }
}
