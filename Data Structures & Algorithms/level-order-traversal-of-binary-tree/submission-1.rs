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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {

        let mut queue = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();

        if let Some(node) = root {
            queue.push_back(node);
        }


        while !queue.is_empty(){
            let level_size = queue.len();
            let mut level_node = Vec::with_capacity(level_size);
            

            for _ in 0..level_size {

                 let node = queue.pop_front().unwrap();
            let borrowed_node = node.borrow();

                   level_node.push(borrowed_node.val);

              if let Some(left_node) = borrowed_node.left.clone() {
                    queue.push_back(left_node);
              }

              if let Some(right_node) = borrowed_node.right.clone() {
                    queue.push_back(right_node);
              }
            }

             result.push(level_node);

           
        }


result




    }
}
