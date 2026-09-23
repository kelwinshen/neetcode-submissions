impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut modified_nums = nums;
        modified_nums.sort();

        println!("{:?}", modified_nums);

        if modified_nums.len() == 0 || modified_nums.len() == 1 {
            return modified_nums.len() as i32;
        } 

        let mut last_number = modified_nums[0];

        let mut max = 1;
        let mut count = 1;
        for i in 1..modified_nums.len(){
            if modified_nums[i] - last_number == 1{
                
                count+=1;
                if count > max {
                    max = count;
                }
            } else if (modified_nums[i] - last_number == 0){
                //do nothing
            }else {
                count = 1;
            }
             last_number = modified_nums[i];
        }
    max
    }
}
