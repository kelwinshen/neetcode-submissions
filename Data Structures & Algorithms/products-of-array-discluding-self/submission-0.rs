impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        //product left * product right = the product of except self 

        //Input: nums = [1,2,4,6]
        //Output: [48,24,12,8]

        // product_except_self = left product * right product 
        // left product of i = left default * nums[0] .... nums[i-1]
        // right profuct of i = right default * nums[nums.len()] ... nums[i+1]

        let mut left_product_vec = Vec::new();
        let mut right_product_vec = Vec::new();
        let mut result: Vec<i32> = Vec::new();

        let mut left_product = 1;
            let mut right_product = 1;

        for i in 0..nums.len(){
            left_product_vec.push(left_product);
            left_product *= nums[i]; //1, 1, 2, 8
            
            right_product_vec.push(right_product);
             right_product *= nums[nums.len()-1-i];  //1, 6, 24, 48
            
           
        }

        right_product_vec.reverse();

        for i in 0..left_product_vec.len(){
            result.push(left_product_vec[i]*right_product_vec[i]);
        }


        result

       

        
    }
}
