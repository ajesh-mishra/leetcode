use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut a_factors = HashSet::from([1]);
        let mut b_factors = HashSet::from([1]);
        let compute = |n: i32, factors: &mut HashSet<i32>| {
            for i in 1..(n / 2) + 1 {
                if n % i == 0 {
                    factors.insert(i);
                    factors.insert(n / i);
                }
            }
        };

        compute(a, &mut a_factors);
        compute(b, &mut b_factors);

        a_factors
            .intersection(&b_factors)
            .collect::<Vec<&i32>>()
            .len() as _
    }
}

fn main() {
    println!("{}", Solution::common_factors(12, 6));
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::common_factors(25, 30), 2);
        assert_eq!(Solution::common_factors(1, 1), 1);
    }
}
