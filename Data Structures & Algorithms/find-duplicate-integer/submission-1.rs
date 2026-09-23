use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {

        let mut hashset_nums: HashSet<i32> = HashSet::new();

        for num in nums {
            if !hashset_nums.insert(num){
                return num;
            }
        }


        1




        //[3,1,3,4,2]
        //len = 5 , the max is 5
        //find out which character not exist 
    }
}
