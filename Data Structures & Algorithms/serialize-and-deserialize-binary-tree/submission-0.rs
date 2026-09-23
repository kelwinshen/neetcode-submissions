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
use std::collections::VecDeque;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec{
        }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out: Vec<String> = Vec::new();

        Self::serialize_helper(root, &mut out);

         out.join(",")
    }

    fn serialize_helper(root: Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<String>){
        match root {
            Some(node)=>{
                let node_ref = node.borrow();
                out.push(node_ref.val.to_string());
                Self::serialize_helper(node_ref.left.clone(), out);
                  Self::serialize_helper(node_ref.right.clone(), out);
            },
            _=>{
                out.push("#".to_string());
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
              let mut vec_deque: VecDeque<&str> = data.split(',').collect();

             Self::deserialize_helper(&mut vec_deque)

    }

    fn deserialize_helper(tokens: &mut VecDeque<&str>)-> Option<Rc<RefCell<TreeNode>>> {
         let token = tokens.pop_front().unwrap();
          if token == "#" {
            return None;
        }

         let val = token.parse::<i32>().unwrap();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        let left = Self::deserialize_helper(tokens);
        let right = Self::deserialize_helper(tokens);
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;
        Some(node)
    }
}
