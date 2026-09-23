use std::collections::BinaryHeap;
use std::cmp::Reverse;


impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();

     
        for i in 0..points.len() {
            let pair = points[i].clone();
            let value = pair[0] * pair[0] + pair[1] * pair[1];

            heap.push((value, pair)); 


          if heap.len() > k as usize {
            heap.pop();
          }
        }

        println!("{:?}", heap);


      let mut answer: Vec<Vec<i32>> = Vec::new();

      for (value, point) in heap {
          answer.push(point);
      }


    answer




    }
}
