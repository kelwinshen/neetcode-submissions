use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        //in rust, the default is max heap binary so the root is the largest one
        //we cause Reverse to the heap binary to make is min heap binary

        let mut heap_binary: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for num in nums {
            heap_binary.push(Reverse(num));

            if heap_binary.len() > k as usize {
                heap_binary.pop();
            }
        }

        heap_binary.peek().unwrap().0


    }
}
