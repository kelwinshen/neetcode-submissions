impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
            //Input: prices = [10,1,5,6,7,1]
            //Need to buy low. sell high
            //find a biggest delta

            let mut max = 0;

            for i in 0..prices.len(){
                for j in i..prices.len(){
                  let curr = prices[j] - prices[i];
                  println!("{}", curr);
                  if max < curr {
                    max = curr;
                  }
                }
            }

            max
    }
}
