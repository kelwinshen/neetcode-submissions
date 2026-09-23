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
   pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>)->bool{
       Self::helper(root, i32::MIN, i32::MAX)
    }

    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32)->bool{
        match root {
            Some(node)=>{
                let node_val = node.borrow().val;
                if node_val <= min || node_val >= max {
                    return false;
                }

                
//left node, the val become max and min still min
//right node, the val become min and max still max
            return Self::helper(node.borrow().left.clone(), min, node_val) && Self::helper(node.borrow().right.clone(), node_val, max);


            },
            _=> true
        }
    }
}




