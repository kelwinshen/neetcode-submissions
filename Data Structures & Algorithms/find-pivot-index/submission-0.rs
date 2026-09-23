impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        //pivot index is when when sum of left == sum of right 

        let mut cum_sum_from_left: Vec<i32> = Vec::new();
         let mut cum_sum_from_right: Vec<i32> = Vec::new();

        let mut cummulative_sum = 0;
         for i in 0..nums.len(){
            cummulative_sum+=nums[i];
            cum_sum_from_left.push(cummulative_sum);
         }


          let mut cummulative_sum = 0;
         for i in 0..nums.len(){
            cummulative_sum+=nums[nums.len()-i-1];
            cum_sum_from_right.push(cummulative_sum);
         }

         for i in 0..nums.len(){
            if cum_sum_from_left[i] == cum_sum_from_right[nums.len()-1-i]{
                return i as i32;
            }
         }

         -1




    }
}
