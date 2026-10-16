use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut binary_heap_odd: BinaryHeap<i32> = BinaryHeap::new();
         let mut binary_heap_even: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

         for i in 0..nums.len(){
            if nums[i]%2 == 0 {
                binary_heap_even.push(Reverse(nums[i]));
            } else {
                binary_heap_odd.push(nums[i]);
            }
         }

         let mut answer: Vec<i32> = Vec::new();

         for _ in 0..binary_heap_even.len(){
            answer.push(binary_heap_even.pop().unwrap().0);
         }

         for _ in 0..binary_heap_odd.len(){
            answer.push(binary_heap_odd.pop().unwrap());
         }

         answer
    }
}
