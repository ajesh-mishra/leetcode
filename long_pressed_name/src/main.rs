struct Solution {}

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name_array: Vec<&str> = name.split("").collect();
        let typed_array: Vec<&str> = typed.split("").collect();

        let name_len = name_array.len();
        let typed_len = typed_array.len();

        let mut i = 1;
        let mut j = 1;

        while i < name_len {
            if name_array[i] == "" {
                break;
            }
            if name_array[i] == typed_array[j] {
                i += 1;
                j += 1;
            } else {
                if typed_array[j] != typed_array[j-1] {
                    return false;
                }
                j += 1;
            }
            if j >= typed_len {
                return false;
            }
        }

        while j < typed_len {
            if typed_array[j] != name_array[name_len - 2] && typed_array[j] != "" {
                return false;
            }
            j += 1;
        }

        true
    }
}

fn main() {
    let name = String::from("alex");
    let typed = String::from("aalexxr");
    println!("{}", Solution::is_long_pressed_name(name, typed));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_true() {
        let name = String::from("alex");
        let typed = String::from("aalexx");
        assert!(Solution::is_long_pressed_name(name, typed));

        let name = String::from("alex");
        let typed = String::from("aaleex");
        assert!(Solution::is_long_pressed_name(name, typed));

        let name = String::from("alex");
        let typed = String::from("aallexxxxxx");
        assert!(Solution::is_long_pressed_name(name, typed));

        let name = String::from("leelee");
        let typed = String::from("lleeelee");
        assert!(Solution::is_long_pressed_name(name, typed));
    }

    #[test]
    fn ut_false() {
        let name = String::from("alex");
        let typed = String::from("aale");
        assert!(!Solution::is_long_pressed_name(name, typed));

        let name = String::from("alex");
        let typed = String::from("aalexxr");
        assert!(!Solution::is_long_pressed_name(name, typed));

        let name = String::from("alex");
        let typed = String::from("balexxr");
        assert!(!Solution::is_long_pressed_name(name, typed));

        let name = String::from("saeed");
        let typed = String::from("ssaaedd");
        assert!(!Solution::is_long_pressed_name(name, typed));

        let name = String::from("xnhtq");
        let typed = String::from("xhhttqq");
        assert!(!Solution::is_long_pressed_name(name, typed));

        let name = String::from("alex");
        let typed = String::from("aaleelx");
        assert!(!Solution::is_long_pressed_name(name, typed));
    }
}
