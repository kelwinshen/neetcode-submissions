impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

       let mut dummy_box = Box::new(ListNode::new(0));

        let mut curr = dummy_box.as_mut();

       while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                let next_node = list1.as_mut().unwrap().next.take();
                curr.next = list1;
                list1 = next_node;
            } else {
                  let next_node = list2.as_mut().unwrap().next.take();
                curr.next = list2;
                list2 = next_node;
            }

            curr = curr.next.as_mut().unwrap();
       }

       if list1.is_some(){
            curr.next = list1;
       } else {
            curr.next = list2;
       }

    dummy_box.next
    }
}