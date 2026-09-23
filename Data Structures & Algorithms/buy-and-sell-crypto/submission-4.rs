impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
            //Input: prices = [10,1,5,6,7,1]
            //Need to buy low. sell high
            //find a biggest delta


            let mut max = 0;

           let mut left_pointer: usize = 0;
           let mut right_pointer: usize = 1;

           while right_pointer < prices.len(){
            if  prices[right_pointer] > prices[left_pointer] {
               let curr = prices[right_pointer] - prices[left_pointer];
               if curr >  max {
                    max = curr;
               }
                
            } else {
                left_pointer = right_pointer;
            }

            right_pointer+=1;

           }

            max
    }
}
