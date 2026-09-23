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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
     if Solution::is_same(root.clone(), sub_root.clone()) {
          return true;
     }
      if let Some(node) = root {
         if Solution::is_same(node.borrow().left.clone(), sub_root.clone()) {
          return true;
         } else if Solution::is_same(node.borrow().right.clone(), sub_root.clone()){
          return true;
         } else {
          return Solution::is_subtree(node.borrow().right.clone(), sub_root.clone()) || Solution::is_subtree(node.borrow().left.clone(), sub_root.clone());
         }
      }
       false
    }

    pub fn is_same(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>)->bool{
        
        if root1.is_none() && root2.is_none(){
            return true;
        }

        if let Some(node1) = &root1 && let Some(node2) = &root2 {
            if node1.borrow().val != node2.borrow().val {
                return false;
            } else {
                return Solution::is_same(node1.borrow().left.clone(), node2.borrow().left.clone()) && 
                 Solution::is_same(node1.borrow().right.clone(), node2.borrow().right.clone());
            }
        }

        false
     }


}
