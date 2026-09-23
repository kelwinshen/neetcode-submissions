use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        //find the freq of the each num
        //once we find a num that has freq more than 1, we return it the value

        let mut hashset: HashSet<i32> = HashSet::new();

        for num in nums{
           if hashset.get(&num).is_some(){
            return num;
           } else {
           hashset.insert(num);
        }
        }
        

        0
    
    }
}
