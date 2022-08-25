struct Solution {}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut longitude = 0;
        let mut latitude = 0;

        for m in moves.chars() {
            match m {
                'U' => longitude += 1,
                'D' => longitude -= 1,
                'R' => latitude += 1,
                'L' => latitude -= 1,
                _ => {}
            }
        }

        longitude == 0 && latitude == 0
    }
}

fn main() {
    let moves = String::from("UD");
    println!("{}", Solution::judge_circle(moves));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let moves = String::from("UD");
        assert!(Solution::judge_circle(moves));
    }

    #[test]
    fn ut_false() {
        let moves = String::from("LL");
        assert!(!Solution::judge_circle(moves));
    }
}
