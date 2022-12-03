struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if original.len() != m * n {
            return vec![];
        }
        let mut result = vec![];
        for row in original.chunks(n) {
            let row_vec = row.iter().map(|&x| x).collect::<Vec<i32>>();
            result.push(row_vec);
        }

        result
    }
}

fn main() {
    let original = vec![1, 2, 3, 4];
    println!("{:?}", Solution::construct2_d_array(original, 2, 2));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let original = vec![1, 2, 3];
        assert_eq!(
            Solution::construct2_d_array(original, 1, 3),
            vec![vec![1, 2, 3]]
        );
    }
}
