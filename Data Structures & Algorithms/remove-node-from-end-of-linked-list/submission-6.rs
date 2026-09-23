impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // 1. Count the nodes (Your exact logic, just slightly more idiomatic)
        let mut count = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            count += 1;
            curr = &node.next;
        }

        // 2. Edge Case: If n == count, we are removing the very first node.
        if count == n {
            // We just take the head's next node and return it, dropping the old head.
            return head.unwrap().next; 
        }

        // 3. Find the node right BEFORE the one we want to remove
        let mut curr_mut = &mut head;
        let target_idx = count - n; 
        
        for _ in 0..(target_idx - 1) {
            if let Some(node) = curr_mut {
                curr_mut = &mut node.next;
            }
        }

        // 4. The "Scissors" Strategy
        // curr_mut is now sitting right before the node we want to delete.
        if let Some(node) = curr_mut {
            // Take ownership of the node to be deleted
            let mut node_to_delete = node.next.take(); 
            
            // If it exists, take ITS next node, and attach it to our current node
            if let Some(mut deleted) = node_to_delete {
                node.next = deleted.next.take();
            }
        }

        head
    }
}