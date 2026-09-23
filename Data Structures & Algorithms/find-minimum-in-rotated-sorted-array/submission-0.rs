impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {

        let mut left_pointer = 0;
        let mut right_pointer = nums.len()-1;
       
        while left_pointer < right_pointer {
             let  k = left_pointer + (right_pointer - left_pointer)/2;
            if nums[k] > nums[right_pointer] {
                 left_pointer = k + 1;
            } else {
                right_pointer = k;
            }
        }

        



        nums[left_pointer]
        



    }
}
