use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|x| x[0]);
        let mut prev: Option<Vec<i32>> = None;
        let mut arrow = 0;
        for point in points {
            if let Some(p) = prev.clone() {
                if point[0] <= p[1] {
                    prev = Some(vec![point[0], min(p[1], point[1])]);
                } else {
                    arrow += 1;
                    prev = Some(point);
                }
            } else {
                prev = Some(point);
            }
        }
        arrow + 1
    }
}

fn main() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    println!("{}", Solution::find_min_arrow_shots(points));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(points), 4);
    }

    #[test]
    fn ut_3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }

    #[test]
    fn ut_2() {
        let points = vec![
            vec![3, 9],
            vec![7, 12],
            vec![3, 8],
            vec![6, 8],
            vec![9, 10],
            vec![2, 9],
            vec![0, 9],
            vec![3, 9],
            vec![0, 6],
            vec![2, 8],
        ];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }
}
