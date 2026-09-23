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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Edge case: empty tree
        if root.is_none() {
            return None;
        }

        // 1. Initialize our "conveyor belt"
        let mut queue = VecDeque::new();
        queue.push_back(root.clone().unwrap());

        // 2. Process the queue until it's empty
        while let Some(node) = queue.pop_front() {
            let mut node_ref = node.borrow_mut();
            
            // TODO: Swap the left and right children of node_ref
            // Hint: You can just use std::mem::swap(&mut node_ref.left, &mut node_ref.right);
            let mut left = node_ref.left.clone();
            let mut right = node_ref.right.clone();

            node_ref.left = right.clone();
             node_ref.right = left.clone();

             if left.is_some(){
                  queue.push_back(left.unwrap());
             }

              if right.is_some(){
                  queue.push_back(right.unwrap());
             }


            // TODO: If the left child exists, push it to the back of the queue
            
            // TODO: If the right child exists, push it to the back of the queue
        }

        root
    }
}
