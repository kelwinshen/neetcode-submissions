impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
       
       let mut read = 0;
       let mut write = 0;

       while read < nums.len() {
            if nums[read] != val {
                nums[write] = nums[read];
                write+=1;
            }
            read+=1;
       }

       write as i32

       
    }
}
