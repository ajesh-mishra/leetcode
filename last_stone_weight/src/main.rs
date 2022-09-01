struct Solution {}

impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        while stones.len() > 1 {
            stones.sort_by(|a, b| a.cmp(&b));

            let last = stones.pop().unwrap();
            let last_1 = stones.pop().unwrap();

            if last > last_1 {
                stones.push(last - last_1);
            }
        }

        *stones.get(0).unwrap_or(&0)
    }
}

fn main() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    println!("{}", Solution::last_stone_weight(stones));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let stones = vec![1];
        assert_eq!(Solution::last_stone_weight(stones), 1);

        let stones = vec![2, 2];
        assert_eq!(Solution::last_stone_weight(stones), 0);
    }
}