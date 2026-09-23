impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        // change the nums to mutable
        //sort 
        //define max to track the maximum consecutive
        
        if nums.len() == 0 || nums.len() == 1 {
            return nums.len()  as i32;
        } 

        nums.sort();

        let mut max: i32 = 1;
        let mut current_count: i32 = 1;

        for i in 1..nums.len() {
           if nums[i] - nums[i-1] == 1{
                current_count+=1;
                if current_count > max {
                    max = current_count;
                }
           } else if nums[i] - nums[i-1] == 0 {
            //do nothing
             
           } else {
                   current_count = 1;
           }
        }

        max


    }
}
