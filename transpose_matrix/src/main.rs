struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let i_len = matrix.len();
        let j_len = matrix[0].len();
        let mut result = Vec::new();

        for j in 0..j_len {
            let mut temp = Vec::new();
            for i in 0..i_len {
                temp.push(matrix[i][j]);
            }
            result.push(temp);
        }

        result
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:#?}", Solution::transpose(matrix));
}
