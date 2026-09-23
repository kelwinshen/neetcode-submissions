use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
struct MedianFinder{
 pub small: BinaryHeap<i32>, //hold the smaller value: Max Heap
    pub large: BinaryHeap<Reverse<i32>> //hold the larger value: Min Heap
   
}

impl MedianFinder{

    pub fn new()->Self{
        MedianFinder{
           small: BinaryHeap::new(),
            large: BinaryHeap::new()
        }
    }
    
    pub fn add_num(&mut self, number: i32){
        //always insert to small one first
        self.small.push(number);
        
        if let (Some(&s_max), Some(&Reverse(l_min))) = (self.small.peek(), self.large.peek()) { 
            if s_max > l_min {
                self.small.pop();
                self.large.push(Reverse(s_max));
            }
        }
        
        //rebalance sizes so they differ by at most 1
        
        if self.small.len() > self.large.len() + 1 {
        self.large.push(Reverse(self.small.pop().unwrap()));
           
            
        } else if self.large.len()  > self.small.len() + 1 {
              self.small.push(self.large.pop().unwrap().0);
              
        }
    }
    
    pub fn find_median(&mut self)->f64{
        if self.small.len() == self.large.len() {
            (self.small.peek().unwrap() + self.large.peek().unwrap().0) as f64/2.0 
        } else if self.small.len() > self.large.len()  {
            *self.small.peek().unwrap() as f64
        } else {
            self.large.peek().unwrap().0 as f64
        }
    }
    
}