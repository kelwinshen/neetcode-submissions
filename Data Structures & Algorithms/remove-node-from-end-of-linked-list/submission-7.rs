// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        //count the length of the list

        let mut head_ref = &head;
        let mut count = 0;
        while let Some(node) = head_ref {
            head_ref = &node.next;
            count+=1;
        }

        println!("{}", count);

        //handle edge case
          if count == n {
            return head.unwrap().next;
        }

        //mathematically expression related to the count and the n

        // Cause n from the back what we need to handle is in mathematic expression
        //remove the m = (count - n + 1) th node 
        //let say [1,2,3,4]
        // count = 4
        // if n = 2 its mean we remove (4-2 + 1) rd = 3 rd node

        let mut head_mut = head.as_mut();

        for _ in 0..(count-n-1){
            if let Some(node) = head_mut {
                let next_node = node.next.as_mut();
                head_mut =  next_node;
            }
        }

         println!("{:?}", head_mut);

        let mut used_node = head_mut.as_mut().unwrap().next.take();

        println!("{:?}", used_node);

       head_mut.unwrap().next = used_node.as_mut().unwrap().next.take();

       head


        
    }
}
