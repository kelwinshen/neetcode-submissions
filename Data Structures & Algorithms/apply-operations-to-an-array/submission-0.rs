impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        //2,2,2,2
        //2 == 2 -> true
        // 4, 0, 2, 2
        // 0 == 2 -> false
        // 4, 0, 2, 2
        // 2 == 2 -> true
        //4, 0, 4 , 0
        //4, 4, 0, 0


        for i in 0..nums.len()-1{
            if nums[i] == nums[i+1] {
                nums[i] = nums[i] * 2;
                nums[i+1] = 0;
            }
        }

        let mut write = 0;
        for read in 0..nums.len(){
            if nums[read] != 0 {
                nums[write] = nums[read];
                write+=1;
            }
        }

        for i in write..nums.len(){
            nums[i] = 0;
        }

        nums
    }
}
