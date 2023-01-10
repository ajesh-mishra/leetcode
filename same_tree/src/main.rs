#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut p_print = String::from("");
        let mut q_print = String::from("");

        fn inner(root: Option<Rc<RefCell<TreeNode>>>, print: &mut String) {
            print.push('-');
            if let Some(node) = root {
                let node = node.borrow();
                print.push_str(format!("{}", node.val).as_str());
                inner(node.left.clone(), print);
                inner(node.right.clone(), print);
            }
        }

        inner(p, &mut p_print);
        inner(q, &mut q_print);
        
        p_print == q_print
    }
}

fn main() {
    println!("{}", Solution::is_same_tree(None, None));
}
