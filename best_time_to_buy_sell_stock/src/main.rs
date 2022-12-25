use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .into_iter()
            .fold((std::i32::MAX, 0), |(low, profit), price| {
                (min(low, price), max(profit, price - low))
            })
            .1
    }
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        let mut lowest = i32::MAX;
        let mut profit = 0;
        for price in prices {
            lowest = min(lowest, price);
            profit = max(profit, price - lowest);
        }
        profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices));
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit_1(prices));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
        let prices = vec![2, 1, 4];
        assert_eq!(Solution::max_profit(prices), 3);
        let prices = vec![2, 4, 1];
        assert_eq!(Solution::max_profit(prices), 2);
        let prices = vec![2, 1, 2, 1, 0, 0, 1];
        assert_eq!(Solution::max_profit(prices), 1);
    }
}
