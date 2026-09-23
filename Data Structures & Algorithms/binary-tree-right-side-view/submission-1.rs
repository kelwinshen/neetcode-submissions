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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        if root.is_none(){
            return Vec::new();
        }
        //length of the anwer will be max deepth of the root
        //in level 0 -> root, push back left then right so we can pop right first
        //in level 1 -> we prioritize from right to left so if answer already has 2 we dont need to push val until next level
        let mut result: Vec<i32> = Vec::new();

        let mut queue = VecDeque::new();

        queue.push_back(root.unwrap());

        let mut curr_height: usize = 1;

        while !queue.is_empty(){

            let mut level_size = queue.len();

            for _ in 0..level_size{
                  let subroot = queue.pop_back();

                 match subroot {
                    Some(node)=>{
                    if result.len() < curr_height {
                        result.push(node.borrow().val);
                    }

                     if node.borrow().right.is_some(){
                        queue.push_front(node.borrow().right.clone().unwrap());
                    }

                     if node.borrow().left.is_some(){
                        queue.push_front(node.borrow().left.clone().unwrap());
                    }

                },
                _=> println!("Do nothing")
            }

            }

           curr_height+=1;
        }

    result
        
    }
}
