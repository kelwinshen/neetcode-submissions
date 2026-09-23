use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::{HashMap,HashSet};

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        let mut hashmap: HashMap<i32, Vec<usize>> = HashMap::new();

        for i in 0..points.len() {
            let pair = points[i].clone();
            let value = pair[0] * pair[0] + pair[1] * pair[1];

            hashmap.entry(value).or_insert(Vec::new()).push(i);

            heap.push(value); 


          if heap.len() > k as usize {
            heap.pop();
          }
        }

        println!("{:?}", heap);


      let mut answer_index: HashSet<usize> = HashSet::new();

      for value in heap {
          answer_index.extend(hashmap.get(&value).unwrap());
      }

    let mut answer = Vec::new();


  for index in answer_index {
    answer.push(points[index].clone());
  }


    answer




    }
}
