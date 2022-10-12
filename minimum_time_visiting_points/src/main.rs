use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut time = 0;
        for coords in points.windows(2) {
            let x = (coords[1][0] - coords[0][0]).abs();
            let y = (coords[1][1] - coords[0][1]).abs();
            time += max(x, y);
        }
        time
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
    println!("{}", Solution::min_time_to_visit_all_points(points));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let points = vec![vec![3, 2], vec![-2, 2]];
        assert_eq!(Solution::min_time_to_visit_all_points(points), 5)
    }
}
