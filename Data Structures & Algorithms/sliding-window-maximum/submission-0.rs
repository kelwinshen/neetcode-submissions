impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        
        let mut start: usize = 0;

        while start+k as usize <= nums.len(){
            let mut max: i32 = i32::MIN;
              for i in start..start+ k as usize{
               if max < nums[i] {
                    max = nums[i];
               }
              }
              result.push(max);
              start+=1;
        }
        
      

        result
            }
}
