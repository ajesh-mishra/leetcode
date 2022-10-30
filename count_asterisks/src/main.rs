struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count = 0;
        for pair in s.split("|").step_by(2) {
            for c in pair.chars() {
                match c {
                    '*' => count += 1,
                    _ => {}
                }
            }
        }
        count
    }
}

fn main() {
    let s = String::from("l|*e*et|c**o|*de|");
    println!("{}", Solution::count_asterisks(s));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let s = String::from("yo|uar|e**|b|e***au|tifu|l");
        assert_eq!(Solution::count_asterisks(s), 5);
        let s = String::from("iamprogrammer");
        assert_eq!(Solution::count_asterisks(s), 0);
    }
}