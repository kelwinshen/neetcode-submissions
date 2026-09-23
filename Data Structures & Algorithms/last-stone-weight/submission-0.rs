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

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();


        for num in stones {
            heap.push(num);
        }

        let mut result = 0;

       loop {

            let first = heap.pop();
        let second = heap.pop();


        match (first, second) {
            (Some(first_value), Some(second_value)) =>{
                let new_value = first_value - second_value;
                heap.push(new_value);
            },
            (Some(first_value), None) =>{
                result = first_value;
                break;
            },
            (None, _)=>{
                 break;
            }
        }


       }


       result

        
        
    }
}
