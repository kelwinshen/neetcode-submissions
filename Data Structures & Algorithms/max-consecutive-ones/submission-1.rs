impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {

        let mut left = 0;
        let mut right = 0;
        let mut max = 0;

      while right < nums.len(){
        if nums[right] == 0 {
            left = right + 1;
        }

        max = max.max(right-left+1);

        right+=1;



      }

      max as i32
          
    }
}
