impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //nums[i] + nums[j] == -nums[k]
        //we can check like nums[k] as the target 

        let mut modified_nums = nums;
        modified_nums.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..modified_nums.len(){
           let  target = -modified_nums[i];
            let mut left_pointer: usize = 0;
              let mut right_pointer: usize = modified_nums.len() - 1;
          
            while left_pointer < right_pointer {
                   if modified_nums[left_pointer] + modified_nums[right_pointer] == target {
                        let mut answer: Vec<i32> = vec![modified_nums[left_pointer], modified_nums[right_pointer], -target];
                        answer.sort();
                         if left_pointer != i && right_pointer != i {
                                result.push(answer);
                        }
                       left_pointer+=1;
                    } else if modified_nums[left_pointer] + modified_nums[right_pointer] > target {
                        right_pointer-=1;
                    } else {
                        left_pointer+=1;
                    }
                }

        }

    
        result.into_iter().collect::<HashSet<_>>().into_iter().collect()

        
    }
}



