impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        //non decreasing -> increasing

        let mut left_pointer = 0;
        let mut right_pointer = numbers.len()-1;

        while left_pointer < right_pointer{
            if numbers[left_pointer] + numbers[right_pointer] == target {
                return vec![left_pointer as i32 + 1, right_pointer as i32 + 1]
            } else if numbers[left_pointer] + numbers[right_pointer] > target{
              right_pointer-=1;
            } else {
                left_pointer+=1;
            }
        }
            Vec::new()
        }
    }

