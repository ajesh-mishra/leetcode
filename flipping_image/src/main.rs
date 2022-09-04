struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(image.len());

        for row in image {
            let mut inv_row: Vec<i32> = Vec::with_capacity(row.len());
            for r in row.iter().rev() {
                match r {
                    1 => inv_row.push(0),
                    0 => inv_row.push(1),
                    _ => {}
                }
            }
            result.push(inv_row);
        }

        result
    }
}

fn main() {
    let image = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
    println!("{:#?}", Solution::flip_and_invert_image(image));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let image = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        assert_eq!(
            Solution::flip_and_invert_image(image),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );

        let image = vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
        ];
        assert_eq!(
            Solution::flip_and_invert_image(image),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
