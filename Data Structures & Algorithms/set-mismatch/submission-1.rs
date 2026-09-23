impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut arr_exist_freq = vec![(false, 0); nums.len()];
        let mut answers: Vec<i32> = Vec::new();

        for i in 0..nums.len(){
            arr_exist_freq[(nums[i]-1) as usize].0 = true;
             arr_exist_freq[(nums[i]-1) as usize].1 += 1;

             if  arr_exist_freq[(nums[i]-1) as usize].1 > 1 {
                answers.push(nums[i]);
             }
        }

        for i in 0..arr_exist_freq.len(){
            if arr_exist_freq[i].0 == false {
                  answers.push(i as i32 + 1);
            }
        }

        answers


    }
}