use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1_hashset: HashSet<i32> = HashSet::new();
        let mut nums2_hashset: HashSet<i32> = HashSet::new();

        for i in 0..nums1.len(){
            nums1_hashset.insert(nums1[i]);
        }

        for i in 0..nums2.len(){
            nums2_hashset.insert(nums2[i]);
        }

        let mut answer: Vec<i32> = Vec::new();

        if nums1_hashset.len() > nums2_hashset.len() {
            for num in nums2_hashset {
                 if nums1_hashset.contains(&num){
                    answer.push(num);
                 }
            }
           
        } else {
             for num in nums1_hashset {
                 if nums2_hashset.contains(&num){
                    answer.push(num);
                 }
            }
        }

        answer
    }
}
