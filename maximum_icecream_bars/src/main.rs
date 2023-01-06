struct Solution {}

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut bars = [0_u32; 100_001];
        let mut coins = coins;
        let mut price = 1;
        let mut count = 0;

        costs.into_iter().for_each(|c| bars[c as usize] += 1);

        while coins >= price && price <= 100_000 {
            while bars[price as usize] > 0 && coins >= price {
                bars[price as usize] -= 1;
                coins -= price;
                count += 1;
            }
            price += 1;
        }
        count
    }
    pub fn max_ice_cream_1(mut costs: Vec<i32>, coins: i32) -> i32 {
        let (mut total, mut count) = (0, 0);
        costs.sort_by(|a, b| a.cmp(b));
        for cost in costs {
            total += cost;
            if total > coins {
                break;
            }
            count += 1;
        }
        count
    }
}

fn main() {
    let costs = vec![1, 3, 2, 4, 1];
    println!("{}", Solution::max_ice_cream(costs, 7));
    let costs = vec![1, 3, 2, 4, 1];
    println!("{}", Solution::max_ice_cream_1(costs, 7));
}
