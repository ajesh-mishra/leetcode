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

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inner(root: Option<Rc<RefCell<TreeNode>>>, mut output: Vec<i32>) -> Vec<i32> {
            if let Some(x) = root {
                let tn = x.borrow();
                output = inner(tn.left.clone(), output);
                output.push(tn.val);
                output = inner(tn.right.clone(), output);
            }

            output
        }

        inner(root, vec![])
    }
}
