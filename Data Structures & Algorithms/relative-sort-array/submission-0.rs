use std::collections::HashSet;
use std::collections::BinaryHeap; 
use std::cmp::Reverse;


impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut binaryheap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
       let mut write: usize = 0;
        for i in 0..arr2.len(){
           for j in 0..arr1.len(){
            if arr1[j] == arr2[i] {
                 arr1[j] = arr1[write];
                arr1[write] = arr2[i];
                write+=1;
            }
           }
        }

        for i in write..arr1.len(){
            binaryheap.push(Reverse(arr1[i]));
        }

         for i in write..arr1.len(){
           arr1[write] = binaryheap.pop().unwrap().0;
           write+=1;
        }


        arr1




    }
}