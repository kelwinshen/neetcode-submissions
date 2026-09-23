impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
       
       //[1,2,3,4]
       //start from index=0
       //result = left product * right product
       //left product = default value = 1
       //right product = default value * 2 * 3 * 4 = 24
       //result = 1 * 24 = 24 

       //start from index=1
        //result = left product * right product
       //left product = default value * 1
       //right product = default value *  3 * 4 = 12
       //result = 1 * 12 = 12 

       //start from index=2
        //result = left product * right product
       //left product = default value * 1 * 2
       //right product = default value  * 4 = 4
       //result = 2 * 4 = 8


   //start from index=3
        //result = left product * right product
       //left product = default value * 1 * 2 * 3 = 6
       //right product = default value  = 1
       //result = 1 * 6 = 6


    //define the left product array
    //define the right product array

    //multiple the element of each array and push it to the answer and at the end we return 

    let mut current_left = 1;
    let mut left_product: Vec<i32> = vec![current_left];

    for i in 0..nums.len() {
        current_left = current_left * nums[i];
        left_product.push(current_left);
    }

    println!("{:?}", left_product);

    //[1, 1, 2, 6, 12]

    let mut current_right = 1;
    let mut right_product: Vec<i32> = vec![current_right];

    for i in 0..nums.len()-1 {
        current_right = current_right * nums[nums.len()-i-1];
        right_product.push(current_right);
    }
   

     let mut answer: Vec<i32> = Vec::new();

     for i in 0..left_product.len()-1{
        answer.push(left_product[i]*right_product[right_product.len()-i-1]);
     }


     answer








       

        
    }
}
