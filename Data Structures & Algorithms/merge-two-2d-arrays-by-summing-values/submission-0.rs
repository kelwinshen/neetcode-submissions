use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut hashmap_nums: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums1.len(){
            *hashmap_nums.entry(nums1[i][0]).or_insert(0) += nums1[i][1];
        }

        for i in 0..nums2.len(){
             *hashmap_nums.entry(nums2[i][0]).or_insert(0) += nums2[i][1];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        for a in hashmap_nums.into_iter(){
            result.push(vec![a.0, a.1]);
        }

        result.sort_by(|a,b| a[0].cmp(&b[0]));
        result
    }
}
