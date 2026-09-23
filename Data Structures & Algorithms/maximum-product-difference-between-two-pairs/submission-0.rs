impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        //find first largest and second largest multiply
        //find first smallest and second smleest multiply
        //substract them

        let mut first_max = i32::MIN;
        let mut second_max = i32::MIN;

        for i in 0..nums.len() {
            if first_max < nums[i] {
                  second_max =  first_max;
                first_max = nums[i];
            } else if second_max < nums[i] {
                second_max =  nums[i];
            }
        }

        println!("{}", first_max);
        println!("{}", second_max);

         let mut first_min = i32::MAX;
        let mut second_min = i32::MAX;

         for i in 0..nums.len() {
            if first_min > nums[i] {
                   second_min = first_min;
                 first_min = nums[i];  
              
            } else if second_min > nums[i] {
                second_min = nums[i];
            }
        }

         println!("{}", first_min);
        println!("{}", second_min);

        (first_max*second_max) - (first_min*second_min)
    }
}