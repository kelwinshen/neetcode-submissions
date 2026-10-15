use std::collections::BinaryHeap; // in default is max heap
use std::cmp::Reverse;
//means the root is max

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut binary_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for i in 0..nums1.len()-n as usize{
            binary_heap.push(Reverse(nums1[i].clone()));
        }

         for i in 0..nums2.len(){
            binary_heap.push(Reverse(nums2[i].clone()));
        }
        let mut count = 0;
        while count < nums1.len() {
            nums1[count] = binary_heap.pop().unwrap().0;
            count+=1;
        }

    }
}
