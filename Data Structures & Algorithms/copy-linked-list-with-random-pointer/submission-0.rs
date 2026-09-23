use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>, 
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }


impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        // Edge Case: If the list is empty, return None.
        if head.is_none() {
            return None;
        }

        // Setup our HashMap. 
        // Key: Raw memory pointer of the OLD node (*const RefCell<Node>)
        // Value: Shared pointer to the brand NEW node (Rc<RefCell<Node>>)
        let mut node_map: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();
        
        // ==========================================
        // PASS 1: Create all new nodes and map them
        // ==========================================
        // We use `.clone()` on the Option to get our own pointer to walk the list.
        let mut curr = head.clone(); 

        while let Some(old_node) = curr {
            // 1. Get the raw memory address of the old node to use as our Key
            let old_ptr = Rc::as_ptr(&old_node);
            
            // 2. Read the value from the old node
            let val = old_node.borrow().val;
            
            // 3. Create a brand new node with that same value (next/random are None for now)
            let new_node = Rc::new(RefCell::new(Node::new(val)));
            
            // 4. Save the mapping! "This old address maps to this new node"
            node_map.insert(old_ptr, new_node);
            
            // 5. Advance our loop to the next node in the original list
            curr = old_node.borrow().next.clone();
        }

        // ==========================================
        // PASS 2: Wire up all the 'next' and 'random' pointers
        // ==========================================
        // Reset our walker back to the head of the old list
        let mut curr = head.clone();

        while let Some(old_node) = curr {
            // 1. Find the raw memory address of the current old node
            let old_ptr = Rc::as_ptr(&old_node);
            
            // 2. Fetch the corresponding NEW node from our map
            // We use Rc::clone so we get a shared pointer to it, rather than moving it out of the map
            let new_node = Rc::clone(node_map.get(&old_ptr).unwrap());
            
            // 3. Wire up the 'next' pointer
            if let Some(old_next) = &old_node.borrow().next {
                let next_ptr = Rc::as_ptr(old_next);
                // Look up the new "next" node, clone its shared pointer, and attach it!
                new_node.borrow_mut().next = Some(Rc::clone(node_map.get(&next_ptr).unwrap()));
            }

            // 4. Wire up the 'random' pointer
            if let Some(old_random) = &old_node.borrow().random {
                let random_ptr = Rc::as_ptr(old_random);
                // Look up the new "random" node, clone its shared pointer, and attach it!
                new_node.borrow_mut().random = Some(Rc::clone(node_map.get(&random_ptr).unwrap()));
            }

            // 5. Advance our loop
            curr = old_node.borrow().next.clone();
        }

        // Finally, return the new head node by looking up the old head's address!
        let head_ptr = Rc::as_ptr(&head.unwrap());
        Some(Rc::clone(node_map.get(&head_ptr).unwrap()))
    }
}