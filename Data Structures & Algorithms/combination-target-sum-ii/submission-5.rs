impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //we can use backtracking to solve this problem 

        candidates.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut current = Vec::new();

        Self::back_tracking(0, &mut current, target, 0, &candidates, &mut result);

        result

    }


    pub fn back_tracking(current_sum: i32, current: &mut Vec<i32>, target: i32,  index: usize, candidates: &Vec<i32>, result: &mut Vec<Vec<i32>> ){

      if current_sum == target {
        result.push(current.clone());
        return;
      }

      if index == candidates.len() || current_sum + candidates[index] > target {
        return;
      }

      current.push(candidates[index]);

      Self::back_tracking(current_sum+candidates[index], current, target, index+1, candidates, result);

      current.pop();

      let mut next_index = index + 1;

      while  next_index < candidates.len() && candidates[index] == candidates[next_index]{
         next_index+=1;
      }

 Self::back_tracking(current_sum, current, target, next_index, candidates, result);


    }
}
