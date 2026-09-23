use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        //store the stones in the binary heap
        //pop the  largest value from the heap and rearrange back the heap
        //pop the second largest value from the heap and rearrange back the heap

        //check the result of largest value - second largest value
        //if the value > 0 we rearrange the value to heap 

        //loop do over again over until the heap is empty or there only 1 stone
        //heap empty mean 0 if dont return the root value

       let mut binary_heap: BinaryHeap<i32> = BinaryHeap::new(); //in default, max

    for num in stones {
        binary_heap.push(num);
    }
    
    while binary_heap.len() > 1 {
         let first_largest = binary_heap.pop().unwrap();
    let second_largest = binary_heap.pop().unwrap();
   
    
    if first_largest > second_largest {
        binary_heap.push(first_largest - second_largest);
    }
    
    }
    
    if binary_heap.len() == 0 {
       return 0;
    }
    
    binary_heap.pop().unwrap()
    
        
        
    }
}
