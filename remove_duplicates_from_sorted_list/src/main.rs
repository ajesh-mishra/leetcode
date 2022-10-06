
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution {}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result_vec = Vec::new();
        while let Some(node) = head {
            result_vec.push(node.val);
            head = node.next;
        }

        result_vec.dedup();
        let mut result = None;

        for &n in result_vec.iter().rev() {
            let mut node = Box::new(ListNode::new(n));
            node.next = result;
            result = Some(node);
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
