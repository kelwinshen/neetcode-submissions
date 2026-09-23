impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Edge case: If list is empty or has 1 node, do nothing.
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        // --- STEP 1: Find the length ---
        let mut len = 0;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        // Find the exact middle cut point
        let mid = (len + 1) / 2; 

        // --- STEP 2: Cut the list in half ---
        let mut curr_mut = head.as_mut();
        // Walk forward to the middle node
        for _ in 0..mid - 1 {
            if let Some(node) = curr_mut {
                curr_mut = node.next.as_mut();
            }
        }
        
        // SCISSORS! We steal the 'next' pointer of the middle node.
        // The first half stays in 'head', and we now own the second half.
        let mut second_half = curr_mut.unwrap().next.take();

        // --- STEP 3: Reverse the second half ---
        let mut prev = None;
        let mut curr = second_half;
        while let Some(mut node) = curr {
            let next_node = node.next.take(); // Detach the next node
            node.next = prev;                 // Point backwards
            prev = Some(node);                // Move prev forward
            curr = next_node;                 // Move curr forward
        }
        second_half = prev; // This is now the head of our reversed second half

        // --- STEP 4: Merge the two halves ---
        // Steal the first half completely out of the mutable reference
        let mut first_half = head.take(); 
        
        // Standard Zipper Dummy setup!
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        
        let mut toggle = true; // Toggle to switch between picking from 1st and 2nd half
        
        while first_half.is_some() || second_half.is_some() {
            if toggle {
                if let Some(mut node) = first_half {
                    first_half = node.next.take();
                    tail.next = Some(node);
                  
                }
            } else {
                if let Some(mut node) = second_half {
                    second_half = node.next.take();
                    tail.next = Some(node);
                 
                }
            }
               tail = tail.next.as_mut().unwrap();
            toggle = !toggle; // Flip the toggle
        }

        // Put the final zipped list back into the original reference!
        *head = dummy.next;
    }
}