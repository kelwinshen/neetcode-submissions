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
   //  You must reverse the first k nodes in the linked list, and then reverse the next k nodes, and so on. If there are fewer than k nodes left, leave the nodes as they are.
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32)->Option<Box<ListNode>>{
        let mut dummy_box = Box::new(ListNode::new(0));
        let mut tail = dummy_box.as_mut();
        
        let mut curr = head;
        
        while Self::node_equal_greater_than_k(&curr, k){
           let mut group_head = curr;
           
           let mut group_tail = &mut group_head;
           
           for _ in 0..k-1{
               group_tail = &mut group_tail.as_mut().unwrap().next;
           }
           
           let next_group = group_tail.as_mut().unwrap().next.take();
           
           let reverse_group = Self::reverse_nodes(group_head);
           
           tail.next = reverse_group;
           
            // 5. Update tail to the end of the group we just reversed
            // (After reversing, the old head is now the tail)
              while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
           
           curr = next_group;
        }
        
        tail.next = curr;
        
       
        
        dummy_box.next
    }
    
    pub fn node_equal_greater_than_k(head: &Option<Box<ListNode>>, k: i32)->bool{
        let mut curr = head.as_ref();
        let mut count = 0;
        while let Some(current_node) = curr {
            curr = current_node.next.as_ref();
            count+=1;
            if count == k as i32 {
                return true;
            }
        }
        
        false
       
    }
    
    pub fn reverse_nodes(head: Option<Box<ListNode>>)->Option<Box<ListNode>>{
        let mut prev = None;
        let mut curr = head;
        
        while let Some(mut current_node) = curr {
            let next_node = current_node.next.take();
            
            current_node.next = prev;
            
            prev = Some(current_node);
            
            curr = next_node;
        }
        
       
        
        prev
    }
}
