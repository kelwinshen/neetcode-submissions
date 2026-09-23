use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            nums.sort();
            let mut current: Vec<i32> = Vec::new();
            let mut result: Vec<Vec<i32>> = Vec::new();
            
      Solution::back_tracking(0, &mut current, &nums,  &mut result);

      result
    }

    pub fn back_tracking(index: usize, current: &mut Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>){

        if index == nums.len(){
            result.push(current.clone());
            return;
        }

        current.push(nums[index]);

        Solution::back_tracking(index+1, current, nums, result);

        current.pop();

          // Choice 2: SKIP candidates[index] — and skip past ALL duplicates of it too
        let mut next_index = index + 1;
        while next_index < nums.len() && nums[next_index] == nums[index] {
            next_index += 1;
        }
        Self::back_tracking(next_index, current, nums, result);


    }
}
