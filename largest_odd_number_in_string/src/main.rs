struct Solution {}

impl Solution {
    pub fn largest_odd_number(mut num: String) -> String {
        let mut digit = || match num.pop() {
            Some(x) => x.to_digit(10),
            None => None,
        };
        while let Some(x) = digit() {
            if x % 2 != 0 {
                return format!("{num}{x}");
            }
        }
        String::from("")
    }
}

fn main() {
    let num = String::from("52");
    println!("{}", Solution::largest_odd_number(num));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let num = String::from("35427");
        assert_eq!(Solution::largest_odd_number(num), String::from("35427"));
        let num = String::from("4206");
        assert_eq!(Solution::largest_odd_number(num), String::from(""));
    }
}
