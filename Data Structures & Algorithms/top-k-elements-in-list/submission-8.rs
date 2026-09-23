use std::collections::{HashMap};
use std::cmp;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_hashmap : HashMap<i32, i32> = HashMap::new();
       
        

        for i in 0..nums.len(){
            *freq_hashmap.entry(nums[i]).or_insert(0) += 1;
              println!("{:?}", freq_hashmap);
        }

       let mut count_vec: Vec<(i32, i32)> = Vec::new();
    for (val, freq) in freq_hashmap {
        count_vec.push((val, freq));
    }

    // 3. Sort the vector by frequency (descending)
    // b.1 is the frequency of 'b', a.1 is the frequency of 'a'.
    // Comparing b to a sorts from Highest to Lowest.
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // 4. Extract the top K values
    let mut result: Vec<i32> = Vec::new();
    for i in 0..(k as usize) {
        result.push(count_vec[i].0); // .0 is the 'val' from our tuple
    }

    result


    }
}
