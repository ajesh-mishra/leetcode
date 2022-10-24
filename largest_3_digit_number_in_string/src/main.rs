use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut result = BinaryHeap::new();
        for (a, (b, c)) in num
            .chars()
            .zip(num.chars().skip(1).zip(num.chars().skip(2)))
        {
            if a == b && b == c {
                result.push(a);
            }
        }
        match result.pop() {
            Some(x) => format!("{x}{x}{x}"),
            None => String::from(""),
        }
    }
}

fn main() {
    let num = String::from("6777133339");
    println!("{}", Solution::largest_good_integer(num));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_some() {
        let num = String::from("6777133339");
        assert_eq!(Solution::largest_good_integer(num), String::from("777"));
        let num = String::from("2300019");
        assert_eq!(Solution::largest_good_integer(num), String::from("000"));
    }

    #[test]
    fn ut_none() {
        let num = String::from("42352338");
        assert_eq!(Solution::largest_good_integer(num), String::from(""));
    }
}
