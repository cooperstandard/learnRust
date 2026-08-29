use std::fmt;

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

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = self;
        let mut values = Vec::new();
        loop {
            values.push(current.val);
            match &current.next {
                Some(node) => current = node,
                None => break,
            }
        }
        write!(f, "[{}]", values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut p = l1.as_deref();
    let mut q = l2.as_deref();
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while p.is_some() || q.is_some() || carry != 0 {
        let x = p.map_or(0, |node| node.val);
        let y = q.map_or(0, |node| node.val);
        let sum = x + y + carry;
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        p = p.and_then(|node| node.next.as_deref());
        q = q.and_then(|node| node.next.as_deref());
    }

    dummy_head.next
}

fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in values.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head.as_deref();
    while let Some(node) = current {
        result.push(node.val);
        current = node.next.as_deref();
    }
    result
}

fn main() {
    let l1 = from_vec(vec![2, 4, 3]);
    let l2 = from_vec(vec![5, 6, 4]);
    let result = add_two_numbers(l1, l2);
    println!("Result: {}", result.as_ref().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let l1 = from_vec(vec![2, 4, 3]);
        let l2 = from_vec(vec![5, 6, 4]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![7, 0, 8]);
    }

    #[test]
    fn example_2() {
        let l1 = from_vec(vec![0]);
        let l2 = from_vec(vec![0]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![0]);
    }

    #[test]
    fn example_3() {
        let l1 = from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = from_vec(vec![9, 9, 9, 9]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn different_lengths() {
        let l1 = from_vec(vec![1, 8]);
        let l2 = from_vec(vec![0]);
        assert_eq!(to_vec(add_two_numbers(l1, l2)), vec![1, 8]);
    }
}
