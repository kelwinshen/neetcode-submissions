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
     pub fn add_two_numbers(head1: Option<Box<ListNode>>,  head2: Option<Box<ListNode>>)->Option<Box<ListNode>>{
        let mut dummy_box: Box<ListNode> = Box::new(ListNode::new(0));
        let mut curr = dummy_box.as_mut();
        let mut curr1 = head1;
        let mut curr2 = head2;
        let mut carry = 0;
        
        while curr1.as_ref().is_some() || curr2.as_ref().is_some() || carry !=0 {
        
          let val1 = if let Some(node) = &curr1 {node.val} else {0};
            let val2 = if let Some(node) = &curr2 {node.val} else {0};
            
            let sum = val1 + val2 + carry;
            
            carry = sum/10;
            
            curr.next = Some(Box::new(ListNode::new(sum%10)));
            
            curr = curr.next.as_mut().unwrap();
            
            if let Some(next_node) = curr1 {
                curr1 = next_node.next;
            }
            
             if let Some(next_node) = curr2 {
                curr2 = next_node.next;
            }
           
        }
        
       
        
       dummy_box.next
        
    }
}

