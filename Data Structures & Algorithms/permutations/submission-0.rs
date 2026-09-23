impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut used: Vec<bool> = vec![false; nums.len()];

        let mut current: Vec<i32> = Vec::new();

        let mut result:Vec<Vec<i32>>  = Vec::new();

        Solution::back_track(&mut used, &mut current, &nums, &mut result);

        result
    }

    pub fn back_track(used: &mut Vec<bool>, current: &mut Vec<i32>,  nums: &Vec<i32>, result: &mut Vec<Vec<i32>>){

        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }


        for i in 0..nums.len(){
            if used[i] {
               continue;
            } 

             current.push(nums[i]);
            used[i] = true;

              Solution::back_track(used, current, nums, result);

            current.pop();
                used[i] = false;
        }

      
      

    }
}
