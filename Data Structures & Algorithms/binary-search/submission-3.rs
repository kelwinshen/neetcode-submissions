impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
        let mut left: usize = 0;
        let mut right: usize = nums.len();

        while left < right {
            let middle_point = left + (right-left)/2;
            if nums[middle_point] == target {
                return middle_point as i32;
            } else if nums[middle_point] > target {
                right = middle_point;
            } else {
                left = middle_point+1;
            }
        }

        return -1;
    }
}
