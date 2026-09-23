impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut current: Vec<i32> = Vec::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        Solution::backtrack(0, 0,&mut current, &nums, &mut result, target);

        result
    }


    pub fn backtrack(index: usize, current_sum: i32 ,current: &mut Vec<i32>, nums: &Vec<i32>, 
    result: &mut  Vec<Vec<i32>> , target: i32){

       
        if current_sum == target {
            result.push(current.clone());
            return;
        }
        if current_sum > target || index == nums.len() {
            return; // pruned: overshot, or ran out of numbers to try
        }

        current.push(nums[index]);

        Solution::backtrack(index, current_sum + nums[index], current, nums, result, target);

        current.pop();

        Solution::backtrack(index+1, current_sum, current, nums, result, target);

    }
}
