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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
     
     

        while lists.len() > 1 {
            let mut new_lists = Vec::new();

             for i in (0..lists.len()).step_by(2){

              let l1 = lists[i].take();
                
                // Get the second list in the pair (Watch out! The number of lists might be odd)
                let l2 = if i + 1 < lists.len() {
                    lists[i + 1].take()
                } else {
                    None
                };

            let result = Self::merge_order_linked_list(l1,l2);

            new_lists.push(result);
        }

        lists = new_lists;

        }

       

    lists.pop()?

    
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
