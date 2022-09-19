use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let alice_total: i32 = alice_sizes.iter().sum();
        let bob_total: i32 = bob_sizes.iter().sum();
        let avg = (alice_total + bob_total) / 2;
        let diff = (avg - bob_total).abs();

        for num in bob_sizes {
            let diff1 = match bob_total.cmp(&avg) {
                Ordering::Greater => num - diff,
                Ordering::Less => num + diff,
                Ordering::Equal => unreachable!()
            };
            if alice_sizes.contains(&diff1) {
                return vec![diff1, num];
            }
        }

        unreachable!()
    }
}

fn main() {
    let alice_sizes = vec![1, 1];
    let bob_sizes = vec![2, 2];
    println!("{:?}", Solution::fair_candy_swap(alice_sizes, bob_sizes));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_bob_greater() {
        let alice_sizes = vec![1, 2];
        let bob_sizes = vec![2, 3];
        assert_eq!(
            Solution::fair_candy_swap(alice_sizes, bob_sizes),
            vec![1, 2]
        );
    }

    #[test]
    fn ut_alice_greater() {
        let alice_sizes = vec![2, 3];
        let bob_sizes = vec![1, 2];
        assert_eq!(
            Solution::fair_candy_swap(alice_sizes, bob_sizes),
            vec![2, 1]
        );
    }
}
