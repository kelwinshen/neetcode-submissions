

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

       let mut write: usize = 0;
       let mut latest: i32 = i32::MIN;
 

       for read in 0..nums.len(){
           if latest != nums[read] {
              nums[write] = nums[read];
              write+=1;
              latest = nums[read];
           }
       }

       write as i32
    }
}
