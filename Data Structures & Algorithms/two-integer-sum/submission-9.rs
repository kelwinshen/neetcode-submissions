use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_hashmap: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..nums.len(){
        index_hashmap.entry(nums[i]).or_insert(Vec::new()).push(i as i32);

         
        }

        let mut modifiable_nums = nums;
        modifiable_nums.sort();

        let mut left: usize = 0;
        let mut right: usize = modifiable_nums.len() - 1;

        while modifiable_nums[left] + modifiable_nums[right] != target {
            if modifiable_nums[left] + modifiable_nums[right] > target {
                right-=1;
            }else{
                left+=1;
            }
        }

       let mut i_list = index_hashmap.get(&modifiable_nums[left]).unwrap().clone();
       let  mut j_list =  index_hashmap.get(&modifiable_nums[right]).unwrap().clone();

       if i_list == j_list {

        return vec![i_list[0], i_list[1]]
       } 

     let mut result =  vec![i_list[0], j_list[0]];


result.sort();

result





    }
}
