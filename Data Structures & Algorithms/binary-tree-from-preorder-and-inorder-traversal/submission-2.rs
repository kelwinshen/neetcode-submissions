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
          //preorder is the boss where live
          //inorder is where we know the position 
          let mut inorder_map: HashMap<i32, usize> = HashMap::new();

          for (i, &val) in inorder.iter().enumerate() {
               inorder_map.entry(val).or_insert(i);
          }

          println!("{:?}", inorder_map);

          let mut preorder_index = 0;

          Self::build(&inorder_map, &preorder, &mut preorder_index, 0, preorder.len()-1)
    }

    pub fn build(
     inorder_map: &HashMap<i32, usize>,
     preorder: &Vec<i32>,
     preoder_index: &mut usize,
     left: usize,
     right: usize,
    )->Option<Rc<RefCell<TreeNode>>>{

     let root_val = preorder[*preoder_index];

     *preoder_index+=1;

     let split_index = inorder_map.get(&root_val).unwrap();

     let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

     if split_index > &left {
          root.borrow_mut().left = Self::build(inorder_map, preorder, preoder_index, left, split_index-1)
     }

       if split_index < &right {
          root.borrow_mut().right = Self::build(inorder_map, preorder, preoder_index,  split_index+1, right)
     }

Some(root)


    }




}
