use std::{rc::Rc, cell::RefCell};
use bt_inorder_traversal::{TreeNode, Solution};

fn main() {
    let t = TreeNode::new(1);
    let s = Solution::inorder_traversal(
        Some(Rc::new(RefCell::new(t)))
    );
    println!("{:?}", s);
    println!("");
}
