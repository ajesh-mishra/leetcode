struct Solution {}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let len = flowerbed.len();

        for i in 0..len {
            if n == 0 { break; }

            if flowerbed[i] == 1 || (i > 0 && flowerbed[i-1] == 1) || (i < len - 1 && flowerbed[i+1] == 1) {
                continue;
            }

            flowerbed[i] = 1;
            n -= 1;
        }

        n == 0
    }
}

fn main() {
    let flowerbed = vec![1, 0, 0, 0, 1];
    println!("{}", Solution::can_place_flowers(flowerbed, 1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        assert!(Solution::can_place_flowers(flowerbed, 1));

        let flowerbed = vec![0, 0, 1, 0, 1];
        assert!(Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn ut_false() {
        // let flowerbed = vec![1, 0, 0, 0, 1];
        // assert!(!Solution::can_place_flowers(flowerbed, 2));

        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 2));
    }
}
