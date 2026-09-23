impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
       let mut result: Vec<Vec<i32>> = Vec::new();
       let mut current: Vec<i32> = Vec::new();
       
       Self::backtrack(0, &mut current, &nums, &mut result);

       result
    }


    fn backtrack(index: usize, current: &mut Vec<i32>, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // Base case: we've made a decision for every element
    if index == nums.len() {
        result.push(current.clone()); // record a COPY of current state
        return;
    }

    // Choice 1: INCLUDE nums[index]
    current.push(nums[index]);        // choose
    Self::backtrack(index + 1, current, nums, result); // explore
    current.pop();                    // un-choose (backtrack!)

    // Choice 2: SKIP nums[index]
    Self::backtrack(index + 1, current, nums, result); // explore, without ever adding it
}
}
