impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 1. Calculate Length
        let mut length = 0;
        let mut curr_ref = head.as_ref();
        while let Some(node) = curr_ref {
            length += 1;
            curr_ref = node.next.as_ref();
        }

        let target_reverse = length - (length % k);
        
        // 2. We use a dummy head to easily attach the start of our list
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        
        // 3. Process groups of k
        let mut current_head = head;
        let mut processed_count = 0;
        
        while processed_count < target_reverse {
            // Take the head of the current group
            let mut group_head = current_head;
            
            // Move current_head forward by k-1 to isolate the chunk
            let mut chunk_tail = &mut group_head;
            for _ in 0..k - 1 {
                chunk_tail = &mut chunk_tail.as_mut().unwrap().next;
            }
            
            // Sever the link to the next group
            current_head = chunk_tail.as_mut().unwrap().next.take();
            
            // Reverse this chunk
            tail.next = Self::reverse_list_node(group_head);
            
            // Advance tail to the end of the reversed chunk
            while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
            
            processed_count += k;
        }
        
        // 4. Attach the remaining nodes that were not reversed
        tail.next = current_head;
        
        dummy.next
    }

    pub fn reverse_list_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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