use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (start, destination) = if start > destination {
            (destination, start)
        } else {
            (start, destination)
        };

        let mut clock = 0;
        let mut anti_clock = 0;

        for (i, &d) in distance.iter().enumerate() {
            let i = i as i32;
            if i >= start && i < destination {
                clock += d;
            } else {
                anti_clock += d;
            }
        }

        min(clock, anti_clock)
    }
}

fn main() {
    let distance = vec![1, 2, 3, 4];
    println!("{}", Solution::distance_between_bus_stops(distance, 0, 3));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_1() {
        let distance = vec![1, 2, 3, 4];
        assert_eq!(Solution::distance_between_bus_stops(distance, 0, 1), 1);
        let distance = vec![1, 2, 3, 4];
        assert_eq!(Solution::distance_between_bus_stops(distance, 0, 2), 3);
        let distance = vec![7,10,1,12,11,14,5,0];
        assert_eq!(Solution::distance_between_bus_stops(distance, 2, 7), 17);
    }

    #[test]
    fn ut_2() {
        let distance = vec![7,10,1,12,11,14,5,0];
        assert_eq!(Solution::distance_between_bus_stops(distance, 7, 2), 17);
    }
}
