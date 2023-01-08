struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        if len == 1 && gas[0] >= cost[0] {
            return 0;
        }
        let mut pos = Vec::with_capacity(len);
        for (i, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            let diff = g - c;
            if diff > 0 {
                pos.push((diff, i));
            }
        }
        pos.sort_by(|a, b| b.0.cmp(&a.0));

        for n in pos.iter().map(|x| x.1) {
            let mut reserve = 0;
            for (i, (g, c)) in gas.iter().zip(cost.iter()).cycle().skip(n).enumerate() {
                reserve += g - c;
                if reserve < 0 {
                    break;
                }
                if i == len - 1 {
                    return n as i32;
                }
            }
        }
        -1
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    println!("{}", Solution::can_complete_circuit(gas, cost));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_positive() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
        let gas = vec![2];
        let cost = vec![2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 0);
    }

    #[test]
    fn ut_negative() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }
}
