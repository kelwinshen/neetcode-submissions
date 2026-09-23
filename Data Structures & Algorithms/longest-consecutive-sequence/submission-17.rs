impl Solution {
   pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {

    if nums.len() == 0 || nums.len() == 1 {
        return nums.len() as i32;
    }
     nums.sort();
     

     let mut current: usize = 1;
     let mut max = 1;
     let mut count = 1;

     
     while current < nums.len(){
     
         if nums[current] == nums[current-1] + 1 {
           count+=1;
            max = (count).max(max);
         } else if nums [current] == nums[current-1] {
            
         } else {
            
             count = 1;
            
         }
           current+=1;
     }
     
     max as i32
     
 }
 
}
