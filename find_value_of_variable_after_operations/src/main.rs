struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for op in operations {
            match op.replace("X", "").as_str() {
                "++" => x += 1,
                "--" => x -= 1,
                _ => {}
            }
        }
        x
    }
}

fn main() {
    let operations = vec!["--X".to_owned(), "X++".to_owned(), "X++".to_owned()];
    println!("{}", Solution::final_value_after_operations(operations));
}
