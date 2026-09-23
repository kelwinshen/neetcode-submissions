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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>,  k: i32) -> i32 {
         let mut order_vec: Vec<i32> = Vec::new();
          Solution::order(root, &mut order_vec);
          order_vec[k as usize-1]
    }

     pub fn order(root: Option<Rc<RefCell<TreeNode>>>,  order_vec: &mut Vec<i32>){
          match root {
               Some(node)=>{
                    Solution::order(node.borrow().left.clone(),  order_vec);

                    order_vec.push(node.borrow().val);

                   
                     Solution::order(node.borrow().right.clone(),  order_vec);
               },
               _=> println!("do nothing")
          }
     }
}
