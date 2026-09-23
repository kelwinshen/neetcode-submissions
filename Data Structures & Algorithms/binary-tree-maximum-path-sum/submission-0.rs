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

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
          


          let (_, max_global) = Self::dfs(&root);

        max_global


    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {

      match node {
        None=>{
          return (0, i32::MIN)
        },

        Some(node)=>{

          let node_ref = node.borrow();

          let val = node_ref.val;

          let (left_branch, left_global) = Self::dfs(&node_ref.left);
          let (right_branch, right_global) = Self::dfs(&node_ref.right);

          let left_clamped = left_branch.max(0);

          let right_clamped = right_branch.max(0);

          let local_v_shape = val + left_clamped + right_clamped;

          let new_global_record = local_v_shape.max(left_global.max(right_global));

           let best_straight_branch = val + left_clamped.max(right_clamped);

           (best_straight_branch, new_global_record)


        }
      }

              
          }

    
}
