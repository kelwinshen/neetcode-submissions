impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let mut left_pointer = 0;
            let mut right_pointer = nums.len() - 1;          

            while left_pointer < right_pointer {
                let k = left_pointer + (right_pointer - left_pointer)/2;

               if nums[k] > nums[right_pointer] {
                    left_pointer = k + 1;
               } else {
                    right_pointer = k;
               }
            }

              let  min_found: (i32, usize) = (nums[left_pointer],left_pointer);


                let mut left_pointer = 0;
            let mut right_pointer = nums.len() - 1;
          

            while left_pointer < right_pointer {
                let k = left_pointer + (right_pointer - left_pointer)/2;

               if nums[k] > nums[right_pointer] {
                    left_pointer = k + 1;
               } else {
                    right_pointer = k;
               }
            }

            let  mut max_found: (i32, usize) =  (nums[nums.len()-1],nums.len()-1);

            if left_pointer != 0 {
                max_found =  (nums[left_pointer-1],left_pointer-1);
            } 

             println!("{:?}", min_found);
            println!("{:?}", max_found);



            // from pointer 0 until max 
            // from pointer min until list end 
            // when special condition both of them is same 

            let mut left_pointer: usize = 0;
            let mut right_pointer: usize = max_found.1+1;

            let mut result_index: i32 = -1;
            while left_pointer < right_pointer {
                let middle_pointer = left_pointer + (right_pointer - left_pointer)/2;
                if nums[middle_pointer] == target {
                    result_index = middle_pointer as i32;
                    break;
                } else if nums[middle_pointer] < target {
                    left_pointer = middle_pointer+1;
                } else {
                    right_pointer = middle_pointer;
                }
            }

            if result_index != -1 {
                return result_index;
            }


            let mut left_pointer: usize = min_found.1;
            let mut right_pointer: usize = nums.len();

            while left_pointer < right_pointer {
                let middle_pointer = left_pointer + (right_pointer - left_pointer)/2;
                if nums[middle_pointer] == target {
                    result_index = middle_pointer as i32;
                    break;
                } else if nums[middle_pointer] < target {
                    left_pointer = middle_pointer+1;
                } else {
                    right_pointer = middle_pointer;
                }
            }

            result_index

           
      }
   

           
    }

