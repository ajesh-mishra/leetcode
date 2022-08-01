struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let binary = format!("{:b}", num);
        let mut result = 0;
        let mut running = 1;

        for n in binary.chars().rev() {
            match n {
                '0' => {
                    result += running;
                }, 
                _ => {}
            }
            running *= 2;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::find_complement(5));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        assert_eq!(Solution::find_complement(1), 0);
    }

    #[test]
    fn tc_2() {
        assert_eq!(Solution::find_complement(2), 1);
    }

    #[test]
    fn tc_3() {
        assert_eq!(Solution::find_complement(5), 2);
    }
}
