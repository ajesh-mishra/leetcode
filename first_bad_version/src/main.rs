pub struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    fn is_bad_version(&self, n: i32) -> bool {
        n >= 1049889538
    }

    pub fn first_bad_version_1(&self, n: i32) -> i32 {
        let mut start = 0;
        let mut end = n;
	    let mut mid = start + (end - start) / 2;

        loop {
            if self.is_bad_version(mid) {
                end  = mid - 1;
                mid = start + (end - start) / 2;
            } else {
                start = mid + 1;
                mid = start + (end - start) / 2;
            }

            if self.is_bad_version(mid) && !self.is_bad_version(mid-1) {
                break;
            }
        }
        
        mid
    }

    fn inner(&self, mut start: i32, mut mid: i32, mut end: i32) -> i32 {
        if self.is_bad_version(mid) {
            end = mid - 1;
            mid = start + (end - start) / 2;
        } else {
            start = mid + 1;
            mid = start + (end - start) / 2;
        }

        if self.is_bad_version(mid) && !self.is_bad_version(mid-1) {
            return mid;
        } else {
            self.inner(start, mid, end)
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        self.inner(0, n/2, n)
    }
}

fn main() {
    let s = Solution::new();
    println!("1049889538 - {}", s.first_bad_version_1(1690815734));
    println!("1049889538 - {}", s.first_bad_version(1690815734));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tc_1() {
        let s = Solution::new();
        assert_eq!(s.first_bad_version_1(1690815734), 1049889538);
    }

    #[test]
    fn tc_2() {
        let s = Solution::new();
        assert_eq!(s.first_bad_version(1690815734), 1049889538);
    }
}
