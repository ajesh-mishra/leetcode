struct Solution {}

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut position = [-1; 26];
        for (i, c) in s.char_indices() {
            let pos = c as usize - 97;
            if position[pos] == -1 {
                position[pos] = i as i32;
            } else {
                let dis = (position[pos] - i as i32).abs() - 1;
                if dis != distance[pos] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let s = String::from("abaccbz");
    let distance = vec![
        1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    println!("{}", Solution::check_distances(s, distance));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_false() {
        let s = String::from("aa");
        let distance = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert!(!Solution::check_distances(s, distance));
    }
}
