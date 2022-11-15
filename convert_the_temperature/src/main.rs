struct Solution {}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin = celsius + 273.15;
        let fahrenheit = celsius * 1.80 + 32.00;
        vec![kelvin, fahrenheit]
    }
}

fn main() {
    println!(
        "[309.65000, 97.70000] \n{:?}",
        Solution::convert_temperature(36.50)
    );
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ut_1() {
        assert_eq!(
            Solution::convert_temperature(122.11),
            vec![395.26000, 251.79800]
        );
    }
}
