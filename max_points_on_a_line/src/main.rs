use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn factor(val: f64) -> i64 {
        (val * 1_000_000_000_000.0).round() as i64
    }
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut hashmap: HashMap<(i64, i64), i32> = HashMap::new();
        for i in 1..points.len() {
            for j in 0..i {
                let x_diff = points[i][0] - points[j][0];
                let y_diff = points[i][1] - points[j][1];
                let (slope, intercept) = if x_diff == 0 {
                    (f64::INFINITY, points[i][0] as f64)
                }
                else {
                    let s = y_diff as f64 / x_diff as f64;
                    let c = points[i][1] as f64 - s * points[i][0] as f64;
                    (s, c)
                };
                let key = (Self::factor(slope), Self::factor(intercept));
                hashmap.entry(key).and_modify(|x| *x += 2).or_insert(2);
            }
        }
        match hashmap.values().max() {
            None => 1,
            Some(&val) => (val as f64).sqrt() as i32 + 1,
        }
    }
}

fn main() {
    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    println!("{}", Solution::max_points(points));
}
