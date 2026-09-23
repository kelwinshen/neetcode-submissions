impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
       
       let mut start: usize = 0;
       let mut end: usize = nums.len();
       let mut result: i32 = -1;

      while start < end  {
            let middle_point = start + (end - start)/2;
            if nums[middle_point] == target {
                result = middle_point as i32;
                break;
            } else if nums[middle_point] < target {
                start = middle_point + 1;
            } else  {
                end = middle_point;
            } 


      }

      result
    }
}
