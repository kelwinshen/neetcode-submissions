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
use std::collections::HashMap;     

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
          //inorder = [left, root, right]
          //preorder = [root, left, right]


          let mut inorder_map : HashMap<i32, usize> = HashMap::new();


          for (i, &val) in inorder.iter().enumerate() {
               inorder_map.entry(val).or_insert(i);
          }

          let mut preorder_index: usize = 0;



          Self::build(
               &preorder,
               &mut preorder_index,
               &inorder_map,
               0,
               preorder.len()-1
          )
    }


    pub fn build(
     preorder: &Vec<i32>,
     preorder_index: &mut usize,
     inorder_map: &HashMap<i32, usize>,
     left: usize,
      right: usize
      )->Option<Rc<RefCell<TreeNode>>>{

          if left > right {
               return None;
          }

          let root_val =  preorder[*preorder_index];

          *preorder_index += 1;
          
         let root_node = Rc::new(RefCell::new(TreeNode::new(root_val)));


        let split_index = *inorder_map.get(&root_val).unwrap();


        if split_index > left {
               root_node.borrow_mut().left = Self::build(preorder, preorder_index, inorder_map, left, split_index-1);
        }


        if split_index < right {
               root_node.borrow_mut().right = Self::build(preorder, preorder_index, inorder_map, split_index+1, right);
        }


Some(root_node)



          
      }
}
