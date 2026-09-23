use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len(){
            *freq.entry(nums[i]).or_insert(0) +=1;
            if *freq.get(&nums[i]).unwrap() > (nums.len()/2) as i32 {
                return nums[i];
            }
        }

        0
    }
}
