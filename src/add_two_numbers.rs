struct Solution {}

// Definition for singly-linked list.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut sum = 0;
        let mut carry = 0;
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
    
        while l1.is_some() || l2.is_some() || carry > 0 {
            let val1 = l1.map_or(0, |node| {
                l1 = node.next.as_ref();
                node.val
            });
    
            let val2 = l2.map_or(0, |node| {
                l2 = node.next.as_ref();
                node.val
            });
    
            sum = val1 + val2 + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }
    
        dummy_head.next
    }
}


fn new_list(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    let mut linked = Box::new(ListNode::new(numbers[0] as i32));

    let mut list = linked.as_mut();

    for i in 1..numbers.len() {
        list.next = Some(Box::new(ListNode::new(numbers[i] as i32)));
        list = list.next.as_mut().unwrap();
    }

    Some(linked)
}
