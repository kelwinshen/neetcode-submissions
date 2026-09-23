use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut arr_freq: HashMap<String, i32> = HashMap::new();
        for i in 0..arr.len(){
            *arr_freq.entry(arr[i].clone()).or_insert(0) +=1;
        }
        
        let mut current_unique_hashset: HashSet<String> = HashSet::new();

        for i in 0..arr.len(){
            if arr_freq.get(&arr[i]).unwrap() == &1 {
                 current_unique_hashset.insert(arr[i].clone());
            } 
           

            if current_unique_hashset.len() == k as usize {
                return arr[i].clone();
            }
        }

        String::new()
    }
}
