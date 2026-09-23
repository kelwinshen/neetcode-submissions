impl Solution {
    fn subsets(nums: Vec<i32>)->Vec<Vec<i32>> {
    let mut current = Vec::new();
    let mut result: Vec<Vec<i32>> = Vec::new();
    
    Self::back_track(&mut current, 0, &nums, &mut result);
    
    result
    
}

fn back_track(current: &mut Vec<i32>, index: usize, nums: &Vec<i32>, result: &mut Vec<Vec<i32>>){
    if index == nums.len(){
        result.push(current.clone()); 
        return;
    }
    
    current.push(nums[index]);
    
    Self::back_track(current, index+1, nums, result);
    
    current.pop();
    
     Self::back_track(current, index+1, nums, result);
    
    
}
}
