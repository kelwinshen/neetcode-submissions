use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>, 
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }


impl Solution {
     pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>)->Option<Rc<RefCell<Node>>>{
        if head.is_none(){
            return None;
        }
        
       //we need to store the relation between the original address and the new node

       let mut map: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();
       
       let mut curr = head.clone(); //just create new pointer of Rc that point to the same specific heap that store the node
       
       while let Some(old_node) = curr {
           let ptr = Rc::as_ptr(&old_node);   //get the raw memory address in heap of original node
           
           let new_node = Rc::new(RefCell::new(Node::new(old_node.borrow().val)));
           
           
           map.insert(ptr,new_node);
           
           curr = old_node.borrow().next.clone();
           
       }
       
       //our new node does not point to any next and random, we need to benchmark with the old node to linked them as the original node
       
       let mut curr = head.clone();
       
       while let Some(old_node) = curr {
           
           let ptr = Rc::as_ptr(&old_node);
           
         let mut new_node = Rc::clone(map.get(&ptr).unwrap());
         
         //wire up the next node
         
         if let Some(old_next) = old_node.borrow().next.as_ref() {
             let next_ptr = Rc::as_ptr(&old_next);
             
             new_node.borrow_mut().next = Some(Rc::clone(map.get(&next_ptr).unwrap()));
             
         }
         
         
         //wire up the random node
         
            if let Some(old_random) = old_node.borrow().random.as_ref() {
                let random_ptr = Rc::as_ptr(&old_random);
                
                new_node.borrow_mut().random = Some(Rc::clone(map.get(&random_ptr).unwrap()));
            }
         
           curr = old_node.borrow().next.clone();
           
       }
       
       
       let head_ptr = Rc::as_ptr(&head.unwrap());
       
       Some(Rc::clone(map.get(&head_ptr).unwrap()))
       
       
        
        
    }
}

