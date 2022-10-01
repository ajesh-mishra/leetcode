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

impl Solution {
    pub fn add_two_numbers_1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }

    fn add(
        node1: Option<Box<ListNode>>,
        node2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if node1.is_none() && node2.is_none() && carry == 0 {
            return None;
        }

        let (val1, next1) = Self::unwrap_node(node1);
        let (val2, next2) = Self::unwrap_node(node2);
        let sum = carry + val1 + val2;
        let next_node = Self::add(next1, next2, sum / 10);

        let sum_node = ListNode {
            val: sum % 10,
            next: next_node,
        };

        Some(Box::new(sum_node))
    }

    fn unwrap_node(node: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
        match node {
            Some(b) => (b.val, b.next),
            None => (0, None),
        }
    }
}

struct Solution {}

impl Solution {
    fn calculate_num(mut link: Option<Box<ListNode>>) -> String {
        let mut num = String::from("");
        while let Some(x) = link {
            let c = char::from_digit(x.val as u32, 10).unwrap();
            num.push(c);
            link = x.next;
        }
        num
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let s1 = Solution::calculate_num(l1);
        let s2 = Solution::calculate_num(l2);
        let mut s1_iter = s1.chars();
        let mut s2_iter = s2.chars();
        let mut carry = 0;
        let mut result = Vec::new();
        let mut break1 = false;
        let mut break2 = false;

        loop {
            let n1 = if let Some(c) = s1_iter.next() {
                c.to_digit(10).unwrap()
            } else {
                break1 = true;
                0
            };
            let n2 = if let Some(c) = s2_iter.next() {
                c.to_digit(10).unwrap()
            } else {
                break2 = true;
                0
            };
            carry = n1 + n2 + carry;
            if carry >= 10 {
                result.push(carry % 10);
                carry = carry / 10;
            } else {
                result.push(carry);
                carry = 0;
            }
            if break1 && break2 {
                break;
            }
        }
        Some(Box::new(ListNode::new(5)))
    }
}

fn main() {
    println!("Hello, world!");
}
