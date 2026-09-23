// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        
        // Edge Case: Empty tree
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        // Process until the conveyor belt is completely empty
        while !queue.is_empty() {
            // THE MAGIC TRICK: Capture the number of nodes currently on this level
            let level_size = queue.len();
            
            // This will hold the values for just this specific level
            let mut current_level_vals = Vec::new();

            // Process EXACTLY `level_size` nodes
            for _ in 0..level_size {
                
                // TODO: 1. Pop the node from the front of the queue
                let mut lower_root = queue.pop_front();

                // TODO: 2. Push the node's value into `current_level_vals`

               match lower_root {
                    Some(node)=>{
                       current_level_vals.push(node.borrow().val);

                     // TODO: 3. If the left child exists, push it to the back of the queue
                
                // TODO: 4. If the right child exists, push it to the back of the queue
                       if node.borrow().left.is_some(){
                              queue.push_back( node.borrow().left.clone().unwrap());
                       }  


                       if node.borrow().right.is_some(){
                           queue.push_back( node.borrow().right.clone().unwrap());
                       }  
                    },
                    _=> println!("Do nothing"),
               }
               
               
            }

            // The level is done! Push the sub-array into our final result
            result.push(current_level_vals)
        }

        result
    }
}