impl Solution {
   fn max_profit(prices: Vec<i32>)->i32{
    let mut max = 0;

    //find most minimum and sell in most maximum thats it 
    
    for i in 0..prices.len(){
        for j in i..prices.len(){
         max = max.max(prices[j] - prices[i]);
        }
    }
    
    max
}
}
