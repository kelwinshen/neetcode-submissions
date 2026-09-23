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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
     
       let mut result: Option<Box<ListNode>> = None;

        for i in 0..lists.len(){
            result = Self::merge_order_linked_list(result, lists[i].clone());
        }


    result

    
    }


       //merge sorted linked list function

    pub fn merge_order_linked_list(mut head1: Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>)->Option<Box<ListNode>>{
        let mut dummy_box = Box::new(ListNode::new(0));

        let mut curr = dummy_box.as_mut();

       while head1.is_some() && head2.is_some() {
            if head1.as_ref().unwrap().val < head2.as_ref().unwrap().val {
                let next_node = head1.as_mut().unwrap().next.take();
                curr.next = head1;
                head1 = next_node;
            } else {
                  let next_node = head2.as_mut().unwrap().next.take();
                curr.next = head2;
                head2 = next_node;
            }

            curr = curr.next.as_mut().unwrap();
       }

       if head1.is_some(){
            curr.next = head1;
       } else {
            curr.next = head2;
       }

    dummy_box.next



    }
}
