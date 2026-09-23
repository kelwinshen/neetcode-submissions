// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut curr: Option<Box<ListNode>> = head;
        let mut prev: Option<Box<ListNode>> = None;

        while let Some(mut current_node) = curr {
            let next_node = current_node.next;
            current_node.next = prev;
            prev = Some(current_node);
            curr = next_node;
        }

    prev

       
    }
}
