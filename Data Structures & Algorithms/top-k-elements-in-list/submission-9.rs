use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> =  HashMap::new();

        for i in 0..nums.len(){
            *freq_map.entry(nums[i]).or_insert(0)+=1;
        }    

        let mut count_tupple_vec: Vec<(i32, i32)> = Vec::new();

        for (value, freq) in freq_map {
            count_tupple_vec.push((value, freq));
        } 

        count_tupple_vec.sort_by(|a,b| {
            let a_freq = a.1;
            let b_freq = b.1;
            b_freq.cmp(&a_freq)
        });

        let mut result: Vec<i32> = Vec::new();

        for i in 0..k as usize{
            result.push(count_tupple_vec[i].0);
        }

        result
        
    }
}
