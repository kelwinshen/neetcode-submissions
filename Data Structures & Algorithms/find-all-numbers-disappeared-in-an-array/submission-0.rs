impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut exist_map = vec![false; nums.len()];

        for i in 0..nums.len(){
          exist_map[nums[i] as usize - 1] = true;
        }

        let mut answers: Vec<i32> = Vec::new();

        for i in 0..exist_map.len() {
            if exist_map[i] == false {
                answers.push(i as i32 + 1);
            }
        }

        answers
    }
}
