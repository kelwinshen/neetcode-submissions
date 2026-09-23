use std::collections::HashMap;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut freq_hashmap: HashMap<i32, i32> = HashMap::new();

        for num in nums{
            *freq_hashmap.entry(num).or_insert(0) +=1;
            if freq_hashmap.get(&num).unwrap_or(&0) > &1 {
                return true;
            }
        }

        return false
    }
}
