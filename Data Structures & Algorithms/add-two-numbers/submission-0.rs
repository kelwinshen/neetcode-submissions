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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut carry = 0;
        let mut curr1 = l1;
        let mut curr2 = l2;
        let mut dummy_box = Box::new(ListNode::new(0));
        let mut curr = dummy_box.as_mut();

         while curr1.as_ref().is_some() || curr2.as_ref().is_some() || carry != 0 {
            let val1 = curr1.as_mut().unwrap_or(&mut Box::new(ListNode::new(0))).val;
              let val2 = curr2.as_mut().unwrap_or(&mut Box::new(ListNode::new(0))).val;

              let sum = val1 + val2 + carry;
              curr.next = Some(Box::new(ListNode::new(sum%10)));
              curr = curr.next.as_mut().unwrap();
              carry = sum/10;

              if let Some(mut node) = curr1 {
                    curr1 = node.next.take();
              }


                if let Some(mut node) = curr2 {
                    curr2 = node.next.take();
              }


            

            

              
        }

       

        dummy_box.next
    }
}
