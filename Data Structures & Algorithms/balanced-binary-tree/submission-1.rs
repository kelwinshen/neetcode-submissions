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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      let (_, is_balance) = Solution::max_deepth_and_is_balance(root);
      is_balance
    }


    pub fn max_deepth_and_is_balance(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        match root {
            Some(node)=>{
                let (left, is_balance_left) = Solution::max_deepth_and_is_balance(node.borrow_mut().left.clone());
                let (right, is_balance_right) = Solution::max_deepth_and_is_balance(node.borrow_mut().right.clone());
                let balance = left.max(right) - left.min(right) <= 1;
                (left.max(right) + 1, balance && is_balance_left && is_balance_right)
            },
            _=>(0, true)
        }
    }
}
