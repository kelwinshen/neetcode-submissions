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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
     
    Solution::max_deepth_with_count_good_nodes(root, i32::MIN)

    }

    pub fn max_deepth_with_count_good_nodes(root:  Option<Rc<RefCell<TreeNode>>>, mut max: i32)->i32{
     let mut count = 0;
     match root {
          Some(node)=>{
               if node.borrow().val >= max {
                    count+=1;
                    max = node.borrow().val;
               }
          count+= Solution::max_deepth_with_count_good_nodes(node.borrow().left.clone(),max);
           count+= Solution::max_deepth_with_count_good_nodes(node.borrow().right.clone(),max);

           count

          },
          _=>0
     }
    }
}
