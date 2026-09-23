impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut current_sum = 0;
        let mut prev = i32::MIN;
        for i in 0..nums.len(){
           if prev < nums[i] {
            current_sum+=nums[i];
           }else{
            current_sum = nums[i];
           }
           prev = nums[i];

          max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}
