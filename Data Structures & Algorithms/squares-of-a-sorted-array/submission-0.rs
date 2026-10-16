use std::collections::BinaryHeap; //max binary heap in default
use std::cmp::Reverse;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut binary_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut write = 0;
        for read in 0..nums.len(){
            binary_heap.push(Reverse(nums[read]*nums[read]));
        }

        for read in 0..nums.len(){
           result.push(binary_heap.pop().unwrap().0);
        }

        result
    }
}
