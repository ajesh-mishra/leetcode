struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut change_5, mut change_10) = (0, 0);

        for i in bills {
            match i {
                5 => {
                    change_5 += 1;
                }
                10 => {
                    change_5 -= 1;
                    change_10 += 1;
                }
                20 => {
                    change_5 -= 1;
                    if change_10 < 1 {
                        change_5 -= 2;
                    } else {
                        change_10 -= 1;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
            if change_5 < 0 || change_10 < 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let bills = vec![5, 5, 5, 10, 20];
    println!("{}", Solution::lemonade_change(bills));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let bills = vec![5, 5, 5, 10, 20];
        assert!(Solution::lemonade_change(bills));

        let bills = vec![
            5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
        ];
        assert!(Solution::lemonade_change(bills));
    }

    #[test]
    fn ut_false() {
        let bills = vec![5, 5, 10, 10, 20];
        assert!(!Solution::lemonade_change(bills));
    }
}
