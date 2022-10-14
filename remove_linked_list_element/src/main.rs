#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn remove_elements_1(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result = Vec::new();
        while let Some(node) = head {
            if node.val != val {
                result.push(node.val);
            }
            head = node.next;
        }
        let mut result_head = None;
        for &i in result.iter().rev() {
            let mut node = Box::new(ListNode::new(i));
            node.next = result_head.clone();
            result_head = Some(node);
        }
        result_head
    }
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut next = &mut head;
        loop {
            match next {
                None => break,
                Some(node) if node.val == val => {
                    *next = node.next.take();
                }
                Some(node) => {
                    next = &mut node.next;
                }
            }
        }
        head
    }
}

fn main() {
    println!("Hello, world!");
}
