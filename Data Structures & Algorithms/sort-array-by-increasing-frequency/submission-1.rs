use std::collections::{HashMap};

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut hashmap_freq: HashMap<i32, i32> = HashMap::new();
        let mut vec_tupple: Vec<(i32, i32)> = Vec::new();

        for i in 0..nums.len(){
            *hashmap_freq.entry(nums[i]).or_insert(0) +=1;
        }

        for (&num, &freq) in hashmap_freq.iter(){
            vec_tupple.push((num, freq));
        }

        vec_tupple.sort_by(|a,b| a.1.cmp(&b.1).then_with(||b.0.cmp(&a.0)));

        let mut answer: Vec<i32> = Vec::new();

 
        for i in 0..vec_tupple.len(){
         
            let freq = vec_tupple[i].1;
            let value = vec_tupple[i].0;

            for j in 0..freq{
                answer.push(value);
            }
        }

        answer


        
    }
}
