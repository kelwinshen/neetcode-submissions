use std::collections::BinaryHeap; //in default max heap, means root is largest number
use std::cmp::Reverse;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {

        let mut heap_binary: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); //min heap

        for i in 0..nums.len() {
            heap_binary.push(Reverse(nums[i]));
        }

        for i in 0..nums.len(){
            nums[i] = heap_binary.pop().unwrap().0;
        }

        nums


       

      
    }
}
