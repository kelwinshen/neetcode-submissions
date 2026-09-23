impl Solution {
   fn combination_sum2(mut candidates: Vec<i32>, target: i32)->Vec<Vec<i32>>{
    candidates.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();
    
    
    Self::back_track(0, &candidates, target, &mut current, 0, &mut result);
    
    result
}

fn back_track(index: usize, candidates: &Vec<i32>, target: i32, current: &mut Vec<i32>, current_sum: i32, result: &mut Vec<Vec<i32>>){
    if current_sum == target {
        result.push(current.clone());
        return;
    }
    
    if  index == candidates.len() || current_sum + candidates[index] > target  {
        return;
    }
    
    current.push(candidates[index]);
    
    Self::back_track(index+1, candidates, target, current, current_sum+candidates[index], result);
    
    current.pop();
    
     let mut next_index = index+1;
     
     while next_index < candidates.len()  && candidates[next_index] == candidates[index] {
         next_index+=1;
     }
     
      Self::back_track(next_index, candidates, target, current, current_sum, result);
     
     
    
}

}
