struct Solution {}

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut result = vec![0; num_people as usize];
        let mut count = 1;
        for pos in (0..num_people as usize).cycle() {
            if count < candies {
                candies -= count;
                result[pos] += count;
            } else {
                result[pos] += candies;
                break;
            }
            count += 1;
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::distribute_candies(7, 4));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        assert_eq!(Solution::distribute_candies(10, 3), vec![5,2,3]);
    }
}