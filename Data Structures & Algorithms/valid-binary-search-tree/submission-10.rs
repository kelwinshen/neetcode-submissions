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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
         fn check(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>)->bool{
           match root {
               Some(node)=>{

                   let val = node.borrow().val;

                    if let Some(min_value) = min {
                        if min_value >= val {
                         return false;
                        }
                    }


                     if let Some(max_value) = max {
                          if max_value <= val{
                         return false;
                          }
                    }

                    let left = check(node.borrow().left.clone(), min, Some(val));
                     let right = check(node.borrow().right.clone(), Some(val), max);
               
 left && right


               },
               _=> true
            }
         }


         check(root, Some(i32::MIN), Some(i32::MAX))
           
    }


}
