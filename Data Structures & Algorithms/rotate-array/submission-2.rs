impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        if k == nums.len() as i32 {
            return;
        }

          k = k  % nums.len() as i32;

        // [1,2,3,4,5,6,7,8] 
        // [1,2,3,4,5,6,7,8]
        //iterate  4 time from back, get the list
        // [5,6,7,8] [1,2,3,4] 


        let mut vec_rotate: Vec<i32> = Vec::new();

        for i in 0..k{
           vec_rotate.push(nums.pop().unwrap());
          
        }
         vec_rotate.reverse();

        vec_rotate.extend(nums.clone());

        *nums = vec_rotate;

        




        
    }
}
