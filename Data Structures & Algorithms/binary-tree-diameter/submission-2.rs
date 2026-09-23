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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

   let (_, max_deepth) = Solution::max_deepth_and_diameter(root);
   max_deepth
        
    }



    pub fn max_deepth_and_diameter(root: Option<Rc<RefCell<TreeNode>>>)->(i32,i32){
      
        match root {
            Some(node)=>{
                let (left, r_left) = Solution::max_deepth_and_diameter(node.borrow_mut().left.clone());
                  let (right, r_right) = Solution::max_deepth_and_diameter(node.borrow_mut().right.clone());
            
                  
                  (left.max(right) + 1, (r_left + r_right).max(left + right))
            },
            _=>(0,0)
        }
    }
}
