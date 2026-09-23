use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    small: BinaryHeap<i32>,          // max-heap: holds the SMALLER half
    large: BinaryHeap<Reverse<i32>>, // min-heap: holds the LARGER half
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Step 1: always push into `small` first
        self.small.push(num);

        // Step 2: make sure every value in `small` is <= every value in `large`
        // (move small's max over to large if it's bigger than large's min)
        if let (Some(&s_max), Some(&Reverse(l_min))) = (self.small.peek(), self.large.peek()) {
            if s_max > l_min {
                let v = self.small.pop().unwrap();
                self.large.push(Reverse(v));
            }
        }

        // Step 3: rebalance sizes so they differ by at most 1
        if self.small.len() > self.large.len() + 1 {
            let v = self.small.pop().unwrap();
            self.large.push(Reverse(v));
        } else if self.large.len() > self.small.len() + 1 {
            let Reverse(v) = self.large.pop().unwrap();
            self.small.push(v);
        }
    }

    pub fn find_median(&mut self) -> f64 {
        if self.small.len() == self.large.len() {
            let &s_top = self.small.peek().unwrap();
            let &Reverse(l_top) = self.large.peek().unwrap();
            (s_top + l_top) as f64 / 2.0
        } else if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            let &Reverse(v) = self.large.peek().unwrap();
            v as f64
        }
    }
}