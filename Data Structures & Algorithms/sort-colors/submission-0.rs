impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut red = 0;

        for i in 0..nums.len(){
            if nums[i] == 0 {
                let temp = nums[red];
                nums[red] = 0;
                nums[i] = temp;
                red+=1;
            }
        }

          let mut white = red;

         for i in 0..nums.len(){
            if nums[i] == 1 {
                 let temp = nums[white];
                nums[white] = 1;
                 nums[i] = temp;
                white+=1;
            }
        }

          let mut blue = white;


         for i in 0..nums.len(){
            if nums[i] == 2 {
                   let temp = nums[blue];
                nums[blue] = 2;
                  nums[i] = temp;
                blue+=1;
            }
        }
    }
}
