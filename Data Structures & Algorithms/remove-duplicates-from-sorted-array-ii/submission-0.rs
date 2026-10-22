impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.push(i32::MIN);
        let mut latest_num = nums[0];
        let mut count = 1;
        let mut write = 0;

        for read in 1..nums.len(){
            if nums[read] == latest_num {
                count+=1;
            } else {
               for k in 0..count{
                if k > 1 {
                    break;
                }
                    nums[write] = latest_num;
                    write+=1;
               }

                count = 1;
            }

            latest_num = nums[read];
        }

        write as i32
    }
}