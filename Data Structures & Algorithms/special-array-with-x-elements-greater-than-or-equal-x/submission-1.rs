impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
       nums.sort();

       let end = nums.last().unwrap().clone();

       for num in 0..=end{
            let mut count = 0;
            for i in 0..nums.len(){
                if num <= nums[i] {
                    count+=1;
                }
            }

            if count == num {
                    return num;
                }
       }

       -1
    }
}