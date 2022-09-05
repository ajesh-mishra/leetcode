struct Solution {}

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        // (y2 – y1)/ (x2 – x1) = (y3 – y2)/ (x3 – x2)

        if points[2][1] == points[1][1] && points[2][0] == points[1][0] {
            return false;
        }

        if points[1][1] == points[0][1] && points[1][0] == points[0][0] {
            return false;
        }

        if points[2][1] == points[0][1] && points[2][0] == points[0][0] {
            return false;
        }

        if points[0][0] == points[1][0] && points[1][0] == points[2][0] {
            return false;
        }

        if points[0][1] == points[1][1] && points[1][1] == points[2][1] {
            return false;
        }

        !((points[1][1] - points[0][1]) as f64 / (points[1][0] - points[0][0]) as f64
            == (points[2][1] - points[1][1]) as f64 / (points[2][0] - points[1][0]) as f64)
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![2, 3], vec![3, 2]];
    println!("{}", Solution::is_boomerang(points));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_positive() {
        let points = vec![vec![1, 1], vec![2, 3], vec![3, 2]];
        assert!(Solution::is_boomerang(points));
    }

    #[test]
    fn ut_negative() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert!(!Solution::is_boomerang(points));

        let points = vec![vec![0, 0], vec![1, 1], vec![1, 1]];
        assert!(!Solution::is_boomerang(points));

        let points = vec![vec![1, 0], vec![1, 1], vec![1, 0]];
        assert!(!Solution::is_boomerang(points));
    }
}
