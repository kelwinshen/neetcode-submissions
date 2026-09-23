use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
     heap: BinaryHeap<Reverse<i32>>,
     k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kl: KthLargest = KthLargest{
            heap: BinaryHeap::new(),
            k: k as usize
        };

        for num in nums {
            kl.add(num);
        }

        kl
        
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));

        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
