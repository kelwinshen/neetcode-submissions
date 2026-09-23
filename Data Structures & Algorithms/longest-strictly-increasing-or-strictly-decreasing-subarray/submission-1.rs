impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
       let mut prev = i32::MIN;
       let mut count = 0;
       let mut max_increasing = 1;
       for i in 0..nums.len(){
            if nums[i] > prev {
                count+=1;
            } else {
                count = 1;
            }

             prev = nums[i];

            max_increasing = count.max(max_increasing);

       }


       let mut prev = i32::MAX;
       let mut count = 0;
       let mut max_decreasing = 1;
       for i in 0..nums.len(){
            if nums[i] < prev {
                count+=1;
            } else {
                count = 1;
            }

             prev = nums[i];

            max_decreasing = count.max(max_decreasing);

       }


       max_increasing.max(max_decreasing)
    }
}
