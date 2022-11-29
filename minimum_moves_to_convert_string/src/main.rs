struct Solution {}

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let (mut count, mut moved) = (0, 0);
        for c in s.chars() {
            match c {
                'X' if moved == 0 => {
                    moved = 2;
                    count += 1;
                }
                _ if moved != 0 => moved -= 1,
                _ => {}
            }
        }
        count
    }
}

fn main() {
    let s = String::from("XXOX");
    println!("{}", Solution::minimum_moves(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("OOOO");
        assert_eq!(Solution::minimum_moves(s), 0);
        let s = String::from("XXX");
        assert_eq!(Solution::minimum_moves(s), 1);
        let s = String::from("OXOX");
        assert_eq!(Solution::minimum_moves(s), 1);
    }
}
