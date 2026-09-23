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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut prev_tail = &mut dummy;
        let mut curr = head;

        // "Walk the belt" - check if we have enough nodes for a full k-group
        while Self::has_k_nodes(&curr, k) {
            // 1. Separate the k-group from the rest of the list
            let mut group_head = curr;
            let mut group_tail = &mut group_head;
            
            // Move group_tail to the k-th node
            for _ in 0..k - 1 {
                group_tail = &mut group_tail.as_mut().unwrap().next;
            }
            
            // 2. Snip the group
            let next_group = group_tail.as_mut().unwrap().next.take();
            
            // 3. Reverse only this group
            let reversed_head = Self::reverse_list_node(group_head);
            
            // 4. Connect to the previous group's tail
            prev_tail.next = reversed_head;
            
            // 5. Update prev_tail to the end of the group we just reversed
            // (After reversing, the old head is now the tail)
            while prev_tail.next.is_some() {
                prev_tail = prev_tail.next.as_mut().unwrap();
            }
            
            // 6. Set curr to the start of the next group
            curr = next_group;
        }

        // Attach the leftovers
        prev_tail.next = curr;
        dummy.next
    }

    fn has_k_nodes(head: &Option<Box<ListNode>>, k: i32) -> bool {
        let mut count = 0;
        let mut curr = head;
        while let Some(node) = curr {
            count += 1;
            if count == k { return true; }
            curr = &node.next;
        }
        false
    }
    
    // Reverse logic (your implementation is perfect!)
    fn reverse_list_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        prev
    }
}
