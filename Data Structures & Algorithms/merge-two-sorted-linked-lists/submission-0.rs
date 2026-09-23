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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head1 = list1;
        let mut head2 = list2;
        let mut anchor: Box<ListNode> = Box::new(ListNode::new(0));
        let mut curr = &mut anchor;

        while head1.is_some() && head2.is_some(){
           if  head1.as_ref().unwrap().val < head2.as_ref().unwrap().val {
                let mut stolen_node = head1.take().unwrap();
                head1 = stolen_node.next.take();
                curr.next = Some(stolen_node);
           } else {
                let mut stolen_node = head2.take().unwrap();
                head2 = stolen_node.next.take();
                curr.next = Some(stolen_node);
           }
           curr = curr.next.as_mut().unwrap();
        }

        if head1.is_some(){
            curr.next = head1.take();
        } else if head2.is_some(){
            curr.next = head2.take();
        }

        anchor.next
    }
}
