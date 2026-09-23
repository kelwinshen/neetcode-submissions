
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct KthLargest{
    pub k: usize,
    pub data: BinaryHeap<Reverse<i32>>
}

impl KthLargest{
    pub fn new(k: i32, nums: Vec<i32>)->Self{
        let mut kl: KthLargest = KthLargest {
            k: k as usize,
            data: BinaryHeap::new()
        };
        
        for num in nums {
            kl.add(num);
        }
        
        kl
    }
    
    pub fn add(&mut self, val: i32)->i32{
      self.data.push(Reverse(val));
        if self.data.len() > self.k as usize {
            self.data.pop();
        }
      
        self.data.peek().unwrap().0
    }
}
