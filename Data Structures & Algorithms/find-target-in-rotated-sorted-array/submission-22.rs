impl Solution {
 fn search(nums: Vec<i32>, target: i32)->i32{
    //using binary search
    let mut left = 0;
    let mut right = nums.len()-1;
    
    while left <= right {
        let middle = left + (right-left)/2;
        
        
        if target == nums[middle] {
            return middle as i32;
        }
        
        //check does the middle point we have in left sorted or right sorted array and do different action toward it
        
        if nums[left] <= nums[middle] {//left
            
            if nums[left] <= target && target < nums[middle] { //does the target is greater than left but less than middle
                right = middle - 1;
            } else {
                left = middle + 1;
            }
            
            
        } else {//right
            if nums[middle] < target && target <= nums[right] { //does the target is less than rightbut greater than middle
                        left = middle + 1;
            } else {
                 right = middle - 1;
            }
        }
        
        
        
        
        
    }
    
    -1
    
    
    
    
}

}