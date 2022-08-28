struct Solution {}

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut result = 0;

        for s in stones.chars() {
            if jewels.contains(s) {
                result += 1;
            }
        }
        
        result
    }
}

fn main() {
    let jewels = String::from("aA");
    let stones = String::from("aAAbbbb");
    println!("{}", Solution::num_jewels_in_stones(jewels, stones));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let jewels = String::from("z");
        let stones = String::from("ZZ");
        assert_eq!(Solution::num_jewels_in_stones(jewels, stones), 0);
    }
}