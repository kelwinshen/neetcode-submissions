use std::collections::HashSet;

impl Solution {
   
fn three_sum(mut nums: Vec<i32>)->Vec<Vec<i32>>{

nums.sort();
    
    // x + y + z = 0;
    //x + y = -z;
        let mut answer = Vec::new();
        let mut answer_set = HashSet::new();
    
    
   for i in 0..nums.len(){
       let target = -nums[i];
       
       let mut left: usize = 0;
       let mut right: usize = nums.len()-1;
       
   
       while  left < right  {
        if nums[left] + nums[right] == target     {
               let mut sub_answer = vec![nums[left], nums[right], -target];
               sub_answer.sort();

              if left != i   && right != i {
                  answer_set.insert(sub_answer);
              }
              left+=1;
              
           } else  if nums[left] + nums[right] > target {
               right-=1;
           } else {
               left+=1;
           }
          
       }
       
      
       
       
   }

   for subanswer in answer_set {
    answer.push(subanswer);
   }

   answer
     
}


}



