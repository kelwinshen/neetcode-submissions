impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        Self::back_track(0, 0, &mut current, &candidates, &mut result, target);
        result
    }

    pub fn back_track(
        index: usize,
        current_sum: i32,
        current: &mut Vec<i32>,
        candidates: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if current_sum == target {
            result.push(current.clone());
            return;
        }
        if index == candidates.len() || current_sum + candidates[index] > target {
            return;
        }

        // Choice 1: INCLUDE candidates[index]
        current.push(candidates[index]);
        Self::back_track(index + 1, current_sum + candidates[index], current, candidates, result, target);
        current.pop();

        // Choice 2: SKIP candidates[index] — and skip past ALL duplicates of it too
        let mut next_index = index + 1;
        while next_index < candidates.len() && candidates[next_index] == candidates[index] {
            next_index += 1;
        }
        Self::back_track(next_index, current_sum, current, candidates, result, target);
    }
}