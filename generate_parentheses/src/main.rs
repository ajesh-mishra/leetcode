struct Solution {}

impl Solution {
    fn dfs(combinations: &mut Vec<String>, str: &mut String, open: usize, close: usize) {
        println!("open: {open}\nclose: {close}\n");
        println!("{str}");
        println!("{combinations:?}\n\n");
        if (open == 0) && (close == 0) {
            combinations.push(str.clone());
            return;
        }
        if open > 0 {
            str.push('(');
            Self::dfs(combinations, str, open - 1, close + 1);
            str.pop();
        }
        if close > 0 {
            str.push(')');
            Self::dfs(combinations, str, open, close - 1);
            str.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut combinations: Vec<String> = vec![];
        let mut str = String::new();
        Self::dfs(&mut combinations, &mut str, n as usize, 0);
        combinations
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
