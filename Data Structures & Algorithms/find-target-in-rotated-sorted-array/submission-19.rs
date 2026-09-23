impl Solution {
   fn search( nums: Vec<i32>, target: i32)->i32{
    
    let mut left = 0;
    let mut right = nums.len() - 1;
    
    while left <= right {
        
        let mid = left + (right-left)/2;
        
        if target == nums[mid] {
            return mid as i32;
        } 
        
        if nums[left] <= nums[mid]{//normal
            
            if nums[left] <= target && target < nums[mid] { //the target is less than mid but greater than left
                  right = mid - 1;
            } else {
               left = mid + 1;
            }
           
            
        } else { //rotated
            
           if nums[mid] < target && target <= nums[right] { //the target is greater than mid but less than right
                   left = mid + 1;
           } else {
               right = mid - 1;
           }
             
            
            
        }
        
        
        
       
        
        
    }
    
    -1
}

}