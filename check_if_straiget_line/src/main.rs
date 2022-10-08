use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn get_slope(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        let (x1, y1) = (a[0], a[1]);
        let (x2, y2) = (b[0], b[1]);
        if y2 == y1 {
            return y1;
        } else if x2 == x1 {
            return x1;
        }
        (y2 - y1) / (x2 - x1)
    }
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut slope_set = HashSet::new();
        for (i, points) in coordinates.windows(2).enumerate() {
            let slope = Self::get_slope(&points[0], &points[1]);
            if i == 0 {
                slope_set.insert(slope);
                continue;
            }
            if slope_set.insert(slope) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let coordinates = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7],
    ];
    println!("{}", Solution::check_straight_line(coordinates));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        assert!(Solution::check_straight_line(coordinates));

        let coordinates = vec![vec![0, 0], vec![0, 1], vec![0, -1]];
        assert!(Solution::check_straight_line(coordinates));
    }

    #[test]
    fn ut_false() {
        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        assert!(!Solution::check_straight_line(coordinates));

        let coordinates = vec![vec![0, 0], vec![0, 5], vec![5, 5], vec![5, 0]];
        assert!(!Solution::check_straight_line(coordinates));
    }
}
